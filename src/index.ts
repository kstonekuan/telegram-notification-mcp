/// <reference path="../worker-configuration.d.ts" />

import { McpAgent } from "agents/mcp";
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import type { ApiResponse, Message } from "@grammyjs/types";

// Helper function to send Telegram messages
async function sendTelegramMessage(
	botToken: string,
	chatId: string | number,
	text: string,
	parseMode?: "Markdown" | "HTML",
	disableNotification?: boolean,
): Promise<Message> {
	const url = `https://api.telegram.org/bot${botToken}/sendMessage`;
	const body = {
		chat_id: chatId,
		text: text,
		parse_mode: parseMode,
		disable_notification: disableNotification,
	};

	const response = await fetch(url, {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(body),
	});

	const result = (await response.json()) as ApiResponse<Message>;

	if (!result.ok) {
		throw new Error(
			`Telegram API error: ${result.description || "Unknown error"}`,
		);
	}

	return result.result;
}

// Define our MCP agent with tools
export class TelegramMCP extends McpAgent {
	server = new McpServer({
		name: "Telegram Notification MCP",
		version: "1.0.0",
	});

	async init() {
		// Send Telegram message tool
		this.server.tool(
			"send_telegram_message",
			{
				text: z.string(),
				chat_id: z.union([z.string(), z.number()]).optional(),
				parse_mode: z.enum(["Markdown", "HTML"]).optional(),
				disable_notification: z.boolean().optional(),
			},
			async ({ text, chat_id, parse_mode, disable_notification }) => {
				const env = this.env as unknown as Env;
				const botToken = env.BOT_TOKEN;
				const defaultChatId = env.DEFAULT_CHAT_ID;

				if (!botToken) {
					return {
						content: [
							{ type: "text", text: "Error: BOT_TOKEN is not configured" },
						],
					};
				}

				const targetChatId = chat_id || defaultChatId;
				if (!targetChatId) {
					return {
						content: [
							{
								type: "text",
								text: "Error: No chat_id provided and DEFAULT_CHAT_ID is not configured",
							},
						],
					};
				}

				try {
					const message = await sendTelegramMessage(
						botToken,
						targetChatId,
						text,
						parse_mode,
						disable_notification,
					);
					return {
						content: [
							{
								type: "text",
								text: `Message sent successfully to chat ${message.chat.id}`,
							},
						],
					};
				} catch (error) {
					return {
						content: [
							{
								type: "text",
								text: `Error sending message: ${error instanceof Error ? error.message : "Unknown error"}`,
							},
						],
					};
				}
			},
		);
	}
}

export default {
	fetch(request: Request, env: Env, ctx: ExecutionContext) {
		const url = new URL(request.url);

		if (url.pathname === "/sse" || url.pathname === "/sse/message") {
			return TelegramMCP.serveSSE("/sse").fetch(request, env, ctx);
		}

		if (url.pathname === "/mcp") {
			return TelegramMCP.serve("/mcp").fetch(request, env, ctx);
		}

		return new Response("Not found", { status: 404 });
	},
};
