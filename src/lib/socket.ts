import { io } from "socket.io-client";

export const socket = io("ws://localhost:6968");

socket.on("connect", () => {
  socket.emit("message", "Hello from React!");
});

socket.on("message-back", (message) => {
  console.log(message);
});
