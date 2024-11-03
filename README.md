# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Development

## Twitch

### Test Websocket eventsub

```bash
twitch event websocket start-server
```

```bash
twitch token -u -s "channel:read:hype_train channel:read:subscriptions moderator:read:followers user:read:email user:read:subscriptions"
```

Trigger examples:

```bash
twitch event trigger channel.follow -t 37530098 --transport=websocket
```
