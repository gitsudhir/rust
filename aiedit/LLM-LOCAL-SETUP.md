# Running LLMs Locally with Ollama

This guide explains how to run a **Large Language Model (LLM) locally** using **Ollama**, and how to interact with it via CLI and HTTP API.

---

## Prerequisites

- macOS
- Minimum **8 GB RAM** (16 GB recommended)
- Internet connection (for first-time model download)

---

## Install Ollama

### macOS
```bash
brew install ollama
```

```bash
ollama --version
```

---

## Start Ollama

Ollama usually starts automatically.
If needed, start it manually:

```bash
ollama serve
```

The API runs on:

```
http://localhost:11434
```

---

## Download and Run a Model

### Pull and run Llama 3

```bash
ollama run llama3
```

List installed models:

```bash
ollama list
```

Example output:

```
NAME             ID              SIZE      MODIFIED
llama3:latest    365c0bd3c000    4.7 GB    2 days ago
```

---

## Generate Text via API

### Streaming (default behavior)

```bash
curl http://localhost:11434/api/generate -d '{
  "model": "llama3",
  "prompt": "Explain LLMs in simple terms"
}'
```

This returns token-by-token streamed output.

---

### Non-Streaming (full response)

```bash
curl http://localhost:11434/api/generate -d '{
  "model": "llama3",
  "prompt": "Explain LLMs in simple terms",
  "stream": false
}'
```

Example response:

```json
{
  "response": "LLMs are computer programs that understand and generate human-like language...",
  "done": true
}
```

---

## Clean Output Only

To extract just the generated text:

```bash
curl -s http://localhost:11434/api/generate -d '{
  "model": "llama3",
  "prompt": "Explain LLMs in simple terms",
  "stream": false
}' | jq -r '.response'
```

---

## Chat-Based API (Conversation Memory)

Use the chat endpoint for multi-turn conversations:

```bash
curl http://localhost:11434/api/chat -d '{
  "model": "llama3",
  "messages": [
    { "role": "system", "content": "You are a helpful assistant" },
    { "role": "user", "content": "Explain LLMs in simple terms" }
  ],
  "stream": false
}'
```

---



## Model Storage Location

* macOS : `~/.ollama/models`


---

## Useful Commands

```bash
ollama list        # list installed models
ollama pull model  # download a model
ollama rm model    # remove a model
ollama stop model  # stop a running model
```

---

## Next Steps

* Build a chat UI (React / Vue / Next.js)
* Integrate with LangChain or LlamaIndex
* Add RAG (Retrieval-Augmented Generation) with local documents
* Deploy as an internal API or developer tool

---

## Status

✅ Ollama running locally
✅ Llama 3 model loaded
✅ API tested successfully

Happy hacking 🚀