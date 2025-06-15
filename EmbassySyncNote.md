# Embassy Sync Note

- [Signal](https://docs.embassy.dev/embassy-sync/git/default/signal/struct.Signal.html) -> Notify consumer On New Value

```mermaid
---
config:
      theme: redux
---
flowchart TD
        A["Producer"]-->B["Consumer"]
```

- [Channel](https://docs.embassy.dev/embassy-sync/git/default/channel/struct.Channel.html) -> bounded size communication, value is removed after first read by consumer, sender will wait if buffer full.

```mermaid
---
config:
      theme: redux
---
flowchart TD
        A["Sender1"]-->B["Channel"]
        C["Sender2"]-->B["Channel"]
        D["Sender3"]-->B["Channel"]
        B-->G["Consumer"]
```

- [PubSubChannel](https://docs.embassy.dev/embassy-sync/git/default/pubsub/struct.PubSubChannel.html)-> its like Channel but multiple consumer. publisher able to wait or force inserting into buffer, when consumer miss data it will receive an error or it can be ignored.

```mermaid
---
config:
      theme: redux
---
flowchart TD
        A["publisher1"]-->B["PubSubChannel"]
        C["publisher2"]-->B["PubSubChannel"]
        D["publisher3"]-->B["PubSubChannel"]
        B-->G["Consumer1"]
        B-->H["Consumer2"]
```

- [Pipe](https://docs.embassy.dev/embassy-sync/git/default/pubsub/struct.PubSubChannel.html)-> `u8` type writer and reader, only one reader but multiple writer.
