import { describe, expect, it } from "vitest";
import { z } from "zod";
import { sendTelegramMessage, type TelegramHttpClient } from "./telegram";

const sentTelegramRequestSchema = z.object({
	chat_id: z.union([z.number(), z.string()]),
	text: z.string(),
	parse_mode: z.enum(["Markdown", "HTML"]).optional(),
	disable_notification: z.boolean().optional(),
});

describe("sendTelegramMessage", () => {
	it("returns the destination chat after Telegram accepts the message", async () => {
		let sentRequestBody: z.infer<typeof sentTelegramRequestSchema> | undefined;
		const acceptingTelegramHttpClient: TelegramHttpClient = async (
			_input,
			requestInitialization,
		) => {
			const rawRequestBody: unknown = JSON.parse(
				String(requestInitialization?.body),
			);
			sentRequestBody = sentTelegramRequestSchema.parse(rawRequestBody);
			return Response.json({
				ok: true,
				result: { chat: { id: 185056757 }, ignoredTelegramField: true },
			});
		};

		const deliveryOutcome = await sendTelegramMessage(
			{
				botToken: "test-bot-token",
				chatId: 185056757,
				text: "Task complete",
				parseMode: "Markdown",
				disableNotification: true,
			},
			acceptingTelegramHttpClient,
		);

		expect(deliveryOutcome).toEqual({ status: "sent", chatId: 185056757 });
		expect(sentRequestBody).toEqual({
			chat_id: 185056757,
			text: "Task complete",
			parse_mode: "Markdown",
			disable_notification: true,
		});
	});

	it("returns an explicit rejection when Telegram declines the message", async () => {
		const rejectingTelegramHttpClient: TelegramHttpClient = async () =>
			Response.json({
				ok: false,
				error_code: 429,
				description: "Too Many Requests",
			});

		const deliveryOutcome = await sendTelegramMessage(
			{
				botToken: "test-bot-token",
				chatId: 185056757,
				text: "Task complete",
			},
			rejectingTelegramHttpClient,
		);

		expect(deliveryOutcome).toEqual({
			status: "rejected",
			errorCode: 429,
			description: "Too Many Requests",
		});
	});

	it("rejects malformed Telegram responses at the network boundary", async () => {
		const malformedTelegramHttpClient: TelegramHttpClient = async () =>
			Response.json({ ok: true, result: {} });

		await expect(
			sendTelegramMessage(
				{
					botToken: "test-bot-token",
					chatId: 185056757,
					text: "Task complete",
				},
				malformedTelegramHttpClient,
			),
		).rejects.toThrow(
			"Telegram API returned a response that does not match the expected contract",
		);
	});
});
