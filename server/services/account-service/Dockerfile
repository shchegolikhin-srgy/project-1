FROM python:3.12-slim as builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends gcc python3-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

COPY requirements.txt .

RUN python -m venv /opt/venv && \
    /opt/venv/bin/pip install --no-cache-dir -U pip setuptools wheel && \
    /opt/venv/bin/pip install --no-cache-dir -r requirements.txt

FROM python:3.11-slim

WORKDIR /app

COPY --from=builder /opt/venv /opt/venv

ENV PATH="/opt/venv/bin:$PATH"

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5 && \
    rm -rf /var/lib/apt/lists/*

COPY . .

ENV PYTHONUNBUFFERED=1 \
    PG_HOST=postgres \
    PG_USER=app_user \
    PG_PASSWORD=your_strong_password \
    PG_DATABASE=app_db

CMD ["gunicorn", "-k", "uvicorn.workers.UvicornWorker", "-c", "python:app.gunicorn_conf", "app.main:app"]