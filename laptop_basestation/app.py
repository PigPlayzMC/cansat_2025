# imports
import numpy as np
import dash
import dash_bootstrap_components as dbc
from dash_bootstrap_templates import load_figure_template
from dash import dcc, Input, Output, html
import plotly.express as px
import pandas as pd


# loading data
CSV_FILE = 'assets/values.csv'
def load_data():
    data = pd.read_csv(CSV_FILE)
    data["time_sec"] = pd.to_numeric(data["time_sec"], errors='coerce')
    data["speed_m_s"] = pd.to_numeric(data["speed_m_s"], errors='coerce')
    data["temperature_C"] = pd.to_numeric(data["temperature_C"], errors='coerce')
    data["altitude_m"] = pd.to_numeric(data["altitude_m"], errors='coerce')
    data["pressure_hPa"] = pd.to_numeric(data["pressure_hPa"], errors='coerce')
    return data 

# initialisation of web app
app = dash.Dash(__name__, external_stylesheets=[dbc.themes.SLATE])
load_figure_template("SIMPLEX_DARK")

# the layout of web app
app.layout = dbc.Container([
    dbc.Row([
        dbc.Col(html.H1("CANSAT Dashboard"), width=12, className="text-center my-5")
    ]),
 
    # stats row
    dbc.Row([
        dbc.Col(html.Div(id="runtime_stats"),    className="text-center my-2"),
        dbc.Col(html.Div(id="speed_stats"),      className="text-center my-2"),
        dbc.Col(html.Div(id="temp_stats"),       className="text-center my-2"),
        dbc.Col(html.Div(id="pressure_stats"),   className="text-center my-2"),
    ], className="mb-5"),

    # check out this ONE single interval yo 
    dcc.Interval(id="goatedinterval", 
                 interval=1000, 
                 n_intervals=0),
 
    dbc.Row([
        dbc.Col(dbc.Card(dbc.CardBody([
            html.H4("Speed readings", className="card-title"),
            dcc.Graph(id="speedgraph"),
        ])), width=6),
 
        dbc.Col(dbc.Card(dbc.CardBody([
            html.H4("Temperature readings", className="card-title"),
            dcc.Graph(id="temperaturegraph"),
        ])), width=6),
 
        dbc.Col(dbc.Card(dbc.CardBody([
            html.H4("Pressure readings", className="card-title"),
            dcc.Graph(id="pressuregraph"),
        ])), width=6),
 
        dbc.Col(dbc.Card(dbc.CardBody([
            html.H4("Altitude readings", className="card-title"),
            dcc.Graph(id="altitudegraph"),
        ])), width=6),
    ]),
], fluid=True)

# the ONE single callback this is getting spicy
@app.callback(
    Output("runtime_stats","children"),
    Output("speed_stats","children"),
    Output("temp_stats","children"),
    Output("pressure_stats","children"),
    Output("speedgraph","figure"),
    Output("temperaturegraph","figure"),
    Output("pressuregraph","figure"),
    Output("altitudegraph","figure"),
    Input("goatedinterval","n_intervals"),
)

# HOPEFULLY it updates in real time now
def update_data(n):

    # this is incase NO data gets sent to the csv yet, so it doesn't break it completely
    try:
        data = load_data()
    except Exception as e:
        print(f"Error loading data: {e}")
        return ["Error loading data"] * 4 + [{}] * 4
 
    if data.empty or data["time_sec"].dropna().empty:
        no_data_message = "No data available yet"
        return [no_data_message] * 4 + [{}] * 4
    
    # calculate avgs and totals
    time_taken = np.max(data["time_sec"])
    avg_speed = np.mean(data["speed_m_s"]).round(1)
    avg_temp = np.mean(data["temperature_C"]).round(1)
    avg_pressure = np.mean(data["pressure_hPa"]).round(1)

    # stats to display
    runtime_stats = f"Total runtime: {time_taken} seconds"
    speed_stats = f"Average speed: {avg_speed} m/s"
    temp_stats = f"Average temperature: {avg_temp} °C"
    pressure_stats = f"Average pressure: {avg_pressure} hPa"

    current_speed = data["speed_m_s"].dropna().tail(1).values[0]
    current_temp = data["temperature_C"].dropna().tail(1).values[0]
    current_pressure = data["pressure_hPa"].dropna().tail(1).values[0]
    current_altitude = data["altitude_m"].dropna().tail(1).values[0]

    fig_speed = px.line(data, x="time_sec", y="speed_m_s",
                        title=f"Current Speed: {current_speed} m/s")
    fig_speed.update_layout(xaxis_title="Time (seconds)", yaxis_title="Speed (m/s)")
 
    fig_temp = px.line(data, x="time_sec", y="temperature_C",
                       title=f"Current Temperature: {current_temp} °C")
    fig_temp.update_layout(xaxis_title="Time (seconds)", yaxis_title="Temperature (°C)")
 
    fig_pressure = px.line(data, x="time_sec", y="pressure_hPa",
                           title=f"Current Pressure: {current_pressure} hPa")
    fig_pressure.update_layout(xaxis_title="Time (seconds)", yaxis_title="Pressure (hPa)")
 
    fig_alt = px.line(data, x="time_sec", y="altitude_m",
                      title=f"Current Altitude: {current_altitude} meters")
    fig_alt.update_layout(xaxis_title="Time (seconds)", yaxis_title="Altitude (meters)")

    return runtime_stats, speed_stats, temp_stats, pressure_stats, fig_speed, fig_temp, fig_pressure, fig_alt

# run the app
if __name__ == "__main__":
    app.run(debug=True)
