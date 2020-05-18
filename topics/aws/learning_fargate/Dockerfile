FROM python:3.8-alpine

RUN pip install gunicorn

COPY app /app
RUN pip install -r /app/requirements.txt

EXPOSE 8000

WORKDIR app

ENTRYPOINT ["gunicorn", "-b", "0.0.0.0:8000", "wsgi:app"]