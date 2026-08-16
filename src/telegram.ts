import type { ApiResponse, Message } from "@grammyjs/types";
import { z } from "zod";

type TelegramMessageReceipt = {
	chat: Pick<Message["chat"], "id">;
};

type TelegramSendMessageApiResponse = ApiResponse<TelegramMessageReceipt>;

const telegramSendMessageApiResponseSchema = z.discriminatedUnion("ok", [
	z.object({
		ok: z.literal(true),
		result: z.object({
			chat: z.object({ id: z.number() }),
		}),
	}),
	z.object({
		ok: z.literal(false),
		error_code: z.number(),
		description: z.string(),
	}),
]) satisfies z.ZodType<TelegramSendMessageApiResponse>;

export type TelegramMessageDeliveryOutcome =
	| { status: "sent"; chatId: Message["chat"]["id"] }
	| { status: "rejected"; errorCode: number; description: string };

export interface TelegramMessageRequest {
	botToken: string;
	chatId: number | string;
	text: string;
	parseMode?: "Markdown" | "HTML";
	disableNotification?: boolean;
}

export type TelegramHttpClient = (
	input: RequestInfo | URL,
	requestInitialization?: RequestInit,
) => Promise<Response>;

export function parseTelegramSendMessageApiResponse(
	rawApiResponse: unknown,
): TelegramSendMessageApiResponse {
	const parseResult =
		telegramSendMessageApiResponseSchema.safeParse(rawApiResponse);
	if (!parseResult.success) {
		throw new Error(
			"Telegram API returned a response that does not match the expected contract",
		);
	}

	return parseResult.data;
}

export async function sendTelegramMessage(
	messageRequest: TelegramMessageRequest,
	telegramHttpClient: TelegramHttpClient = fetch,
): Promise<TelegramMessageDeliveryOutcome> {
	const telegramApiUrl = `https://api.telegram.org/bot${messageRequest.botToken}/sendMessage`;
	const telegramApiResponse = await telegramHttpClient(telegramApiUrl, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify({
			chat_id: messageRequest.chatId,
			text: messageRequest.text,
			parse_mode: messageRequest.parseMode,
			disable_notification: messageRequest.disableNotification,
		}),
	});
	const rawApiResponse: unknown = await telegramApiResponse.json();
	const parsedApiResponse = parseTelegramSendMessageApiResponse(rawApiResponse);

	if (!parsedApiResponse.ok) {
		return {
			status: "rejected",
			errorCode: parsedApiResponse.error_code,
			description: parsedApiResponse.description,
		};
	}

	return {
		status: "sent",
		chatId: parsedApiResponse.result.chat.id,
	};
}
