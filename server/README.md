# Prover Session Manager Server

A Rust-based server that manages prover instances for processing proofs of residency. The server creates and manages individual prover instances for each user session.

## Overview

The server provides endpoints for:

- Creating new prover sessions
- Maintaining session activity through heartbeats
- Cleaning up sessions when they're no longer needed

## Configuration

Copy `.env.example` to `.env` and set the following environment variables:

```env
SP1_PROVER=local        # 'mock', 'local', or 'network'
SP1_PRIVATE_KEY=        # Required if using 'network' mode
SERVER_PORT=8080        # Optional, defaults to 8080
```

## API Endpoints

### Create Session

Creates a new prover instance and returns session details.

```
POST /session

Response 200:
{
    "session_id": "uuid-string",
    "prover_port": 12345
}
```

### Maintain Session

Keeps a session alive by updating its last active timestamp.

```
POST /session/{session_id}/heartbeat

Response 200:
{
    "message": "Session updated"
}
```

### Cleanup Session

Manually cleanup a session and its associated prover instance.

```
DELETE /session/{session_id}

Response 200:
{
    "message": "Session cleaned up successfully"
}
```

## Error Responses

All endpoints may return the following error responses:

```json
{
    "error": "Error message description"
}
```

Common status codes:

- `404`: Session not found
- `503`: Service unavailable (failed to create prover)
- `500`: Internal server error

## Automatic Cleanup

The server automatically cleans up inactive sessions:

- Sessions timeout after 1 hour of inactivity
- Cleanup check runs every 5 minutes
- Clients should call the heartbeat endpoint periodically to maintain active sessions