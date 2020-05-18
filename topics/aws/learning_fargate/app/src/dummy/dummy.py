from flask.blueprints import Blueprint

dummy = Blueprint("dummy", __name__)


@dummy.route("/dummy")
def base_dummy():
    return "Dummy!"
