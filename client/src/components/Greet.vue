<script setup lang="ts">
import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/tauri";

import {
  createPromiseClient,
} from "@bufbuild/connect";
import {
  createGrpcWebTransport,
} from "@bufbuild/connect-web";
import type { PartialMessage } from "@bufbuild/protobuf";
// import { Greeter } from '../../services/greeting_connect';
// import { Person } from '../../services/greeting_pb';
import { Chat } from '../../services/chat_connect';
import { ConnectServerRequest, SendMessageRequest } from '../../services/chat_pb';

const name = ref("");
const content = ref("");

const chatMessages = ref<string[]>([])

// gRPCクライアントの初期化
const transport = createGrpcWebTransport({
  baseUrl: "http://localhost:50051",
});
const client = createPromiseClient(Chat, transport);

// const greet = async () => {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   // greetMsg.value = await invoke("greet", { name: name.value });

//   const person: PartialMessage<Person> = { name: name.value };
//   // gRPCメソッドを呼び出す
//   const greetingMessage = await client.sayHello(person);

//   greetMsg.value = greetingMessage.text;
// }

const connectServer = async () => {
  const req: PartialMessage<ConnectServerRequest> = { userName: name.value }

  const stream = client.connectServer(req);
  for await (const message of stream) {
    const userName = message.userName;
    const content = message.content;

    chatMessages.value.push(`${userName}: ${content}`);
  }
}

const sendMessage = async () => {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // greetMsg.value = await invoke("greet", { name: name.value });

  const req: PartialMessage<SendMessageRequest> = { message: { userName: name.value, content: content.value } };
  // gRPCメソッドを呼び出す
  await client.sendMessage(req);
}


</script>

<template>
  <form class="row" @submit.prevent="connectServer">
    <input id="connect-server-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Connect Server</button>
  </form>

  <form class="row" @submit.prevent="sendMessage">
    <input id="send-message-input" v-model="content" placeholder="Enter a content..." />
    <button type="submit">Send</button>
  </form>

  <div v-for="message in chatMessages">
    <div>{{  message }}</div>
  </div>
</template>
