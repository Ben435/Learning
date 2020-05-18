from src.app import app
# DO NOT TOUCH
# Used in the docker container as the gunicorn target.

if __name__ == "__main__":
    app.run()
