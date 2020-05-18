from flask import Flask
from .dummy.dummy import dummy

app = Flask(__name__)
app.register_blueprint(dummy, url_prefix="/blueprints")


@app.route("/", methods=["GET"])
def index():
    return "Hello world!"


if __name__ == "__main__":
    app.run()
