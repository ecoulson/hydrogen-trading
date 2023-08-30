from flask import Flask
from handlers import simulation_handler

app = Flask(__name__)
app.register_blueprint(simulation_handler.register)
