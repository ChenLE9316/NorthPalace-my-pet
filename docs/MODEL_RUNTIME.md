# Local AI Runtime

## Planned model

MiniCPM5-1B is the initial local model candidate for the Ryzen 3 2200G + 16 GB DRAM target.

Official references:

- Model: https://huggingface.co/openbmb/MiniCPM5-1B
- GGUF: https://huggingface.co/openbmb/MiniCPM5-1B-GGUF
- GGUF files: https://huggingface.co/openbmb/MiniCPM5-1B-GGUF/tree/main
- MiniCPM GitHub: https://github.com/OpenBMB/MiniCPM
- Ollama model: https://ollama.com/openbmb/minicpm5

## Runtime design

```text
northpalace-my-pet.exe
        │
        │ JSON IPC (stdio / Named Pipe)
        ▼
northpalace-llm-worker.exe
        │
        ▼
llama.cpp
        │
        ▼
MiniCPM5-1B GGUF
```

The model worker is not implemented in the first skeleton commit. That is deliberate: the offline Pet Brain must become useful before AI is introduced.

## Quantization baseline

First benchmark candidates:

1. Q8_0 — quality-oriented baseline if memory/latency is acceptable.
2. Q4_K_M — lighter fallback for lower memory pressure.

The final default must be chosen from measured runtime data on the actual Ryzen 3 2200G machine rather than assumption.

## Context policy

The model may support a long context, but NorthPalace-my-pet should use dynamic working contexts. Typical pet interactions should use small context windows plus retrieved memory. Large context is reserved for tasks that genuinely need it.

## Worker lifecycle

Planned states:

- unloaded
- loading
- ready
- busy
- cooling/sleeping
- error

Pet Brain must have a valid behavior path for every worker state.
