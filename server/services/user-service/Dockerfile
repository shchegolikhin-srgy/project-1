FROM python:3.11-slim AS builder

WORKDIR /app

COPY requirements.txt .

RUN pip install --no-cache-dir --upgrade -r requirements.txt --target /app/deps

FROM python:3.11-slim

WORKDIR /app

COPY --from=builder /app/deps /app/deps

ENV PYTHONPATH=/app/deps

COPY . .

EXPOSE 4003

CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "4003"]