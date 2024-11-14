```mermaid
flowchart LR
  classDef Process stroke:#f80
  classDef Library stroke:#0f0
  classDef Authored stroke:#0ff
  classDef Generated stroke:#ff0

  subgraph Legend
      Process[[Compilation/Generation Process]]:::Process
      Library:::Library
      Authored["Manually-written Code"]:::Authored
      Generated["Generated Code"]:::Generated
  end
```