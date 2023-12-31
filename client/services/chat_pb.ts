// @generated by protoc-gen-es v1.3.0 with parameter "target=ts"
// @generated from file chat.proto (package chat, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Message as Message$1, proto3 } from "@bufbuild/protobuf";

/**
 * @generated from message chat.ConnectServerRequest
 */
export class ConnectServerRequest extends Message$1<ConnectServerRequest> {
  /**
   * @generated from field: string user_name = 1;
   */
  userName = "";

  constructor(data?: PartialMessage<ConnectServerRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "chat.ConnectServerRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "user_name", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): ConnectServerRequest {
    return new ConnectServerRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): ConnectServerRequest {
    return new ConnectServerRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): ConnectServerRequest {
    return new ConnectServerRequest().fromJsonString(jsonString, options);
  }

  static equals(a: ConnectServerRequest | PlainMessage<ConnectServerRequest> | undefined, b: ConnectServerRequest | PlainMessage<ConnectServerRequest> | undefined): boolean {
    return proto3.util.equals(ConnectServerRequest, a, b);
  }
}

/**
 * @generated from message chat.Message
 */
export class Message extends Message$1<Message> {
  /**
   * @generated from field: string user_name = 2;
   */
  userName = "";

  /**
   * @generated from field: string content = 3;
   */
  content = "";

  constructor(data?: PartialMessage<Message>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "chat.Message";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 2, name: "user_name", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 3, name: "content", kind: "scalar", T: 9 /* ScalarType.STRING */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Message {
    return new Message().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Message {
    return new Message().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Message {
    return new Message().fromJsonString(jsonString, options);
  }

  static equals(a: Message | PlainMessage<Message> | undefined, b: Message | PlainMessage<Message> | undefined): boolean {
    return proto3.util.equals(Message, a, b);
  }
}

/**
 * @generated from message chat.SendMessageRequest
 */
export class SendMessageRequest extends Message$1<SendMessageRequest> {
  /**
   * @generated from field: chat.Message message = 1;
   */
  message?: Message;

  constructor(data?: PartialMessage<SendMessageRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "chat.SendMessageRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "message", kind: "message", T: Message },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): SendMessageRequest {
    return new SendMessageRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): SendMessageRequest {
    return new SendMessageRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): SendMessageRequest {
    return new SendMessageRequest().fromJsonString(jsonString, options);
  }

  static equals(a: SendMessageRequest | PlainMessage<SendMessageRequest> | undefined, b: SendMessageRequest | PlainMessage<SendMessageRequest> | undefined): boolean {
    return proto3.util.equals(SendMessageRequest, a, b);
  }
}

