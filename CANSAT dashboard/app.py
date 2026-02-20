# imports
import numpy as np
import dash
import dash_bootstrap_components as dbc
from dash import dcc, Input, Output, html
import plotly.express as px
import pandas as pd


# loading data
def load_data():
    data = pd.read_csv('assets/samplevalues.csv')
    data["time_sec"] = pd.to_numeric(data["time_sec"], errors='coerce')
    data["speed_m_s"] = pd.to_numeric(data["speed_m_s"], errors='coerce')
    data["temperature_C"] = pd.to_numeric(data["temperature_C"], errors='coerce')
    data["altitude_m"] = pd.to_numeric(data["altitude_m"], errors='coerce')
    data["pressure_hPa"] = pd.to_numeric(data["pressure_hPa"], errors='coerce')
    return data 


data = load_data()

# calculate avgs and totals
time_taken = len(data)
avg_speed = np.mean(data["speed_m_s"]).round(1)
avg_temp = np.mean(data["temperature_C"]).round(1)
avg_pressure = np.mean(data["pressure_hPa"]).round(1)

# initialisation of web app
app = dash.Dash(__name__, external_stylesheets=[dbc.themes.BOOTSTRAP])

# the layout of web app
app.layout = dbc.Container([
    dbc.Row([
        dbc.Col(html.H1("CANSAT Dashboard"), width=20, className="text-center my-5")
    ]),

    # stats and that
    dbc.Row([
        dbc.Col(html.Div(f"Total runtime: {time_taken} seconds", className="text-center my-2 top-text"), width=20),
        dbc.Col(html.Div(f"Average speed: {avg_speed} m/s", className="text-center my-2 top-text"), width=20),
        dbc.Col(html.Div(f"Average temperature: {avg_temp} °C", className="text-center my-2 top-text"), width=20),
        dbc.Col(html.Div(f"Average pressure: {avg_pressure} hPa", className="text-center my-2 top-text"), width=20),
    ], className="mb-5"),


    dbc.Row([
        dbc.Col([
            dbc.Card([
                dbc.CardBody([
                    html.H4("Speed readings", className="card-title"),
                    dcc.Graph(id="speed-over-time")
                ])
            ])
        ], width=6),

        dbc.Col([
            dbc.Card([
                dbc.CardBody([
                    html.H4("Temperature readings", className="card-title"),
                    dcc.Graph(id="temperature-over-time")
                ])
            ])
        ], width=6),

        dbc.Col([
            dbc.Card([
                dbc.CardBody([
                    html.H4("Pressure readings", className="card-title"),
                    dcc.Graph(id="pressure-over-time")
                ])
            ])
        ], width=6),

        dbc.Col([
            dbc.Card([
                dbc.CardBody([
                    html.H4("Altitude readings", className="card-title"),
                    dcc.Graph(id="altitude-over-time")
                ])
            ])
        ], width=6),
    ]),
], fluid=True)


# callbacks


# run the app
if __name__ == "__main__":
    app.run(debug=True)