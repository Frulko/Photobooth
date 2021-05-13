# Porting documentation

## Classes

### Camera
Purpose
**Properties**
- isConnected:bool
**Methods**
- Init(void)
- Close(void)
- ReConnect(void)
- TakePicture([]byte, bool)
- GeneratePrintable([]byte)
- Print(void)
- Send(int)
- Reset(void)
- GetConfig(void)
- SetConfig(void)

### Image
Purpose
**Properties**
- imageBytes:[]byte
- printableImage:[]byte
- backgroundImage: image.Image ?
**Methods**
- Init(void)
- SetImage(void)
- Prepare([]byte)
- GeneratePrintable([]byte)
- Print(void)
- GetCounter(int)
- GetTemp(String)
- SetCounter(void)
- Reset(void)

### Sender
Purpose
**Properties**
- serverURL: String

**Methods**
- Init(String)
- Upload(int)
- newfileUploadRequest(*http.Request, error)
- formattedTimestamp(String)
- format(String)


### WebSocketHandler
Handle connection from WebSocket and dispatch commands
**Properties**
- isConnected:bool
- email:String

**Methods**
- Init
- ReConnect
- OnConnection
- OnDisconnection
- OnMessage
- sendMessage