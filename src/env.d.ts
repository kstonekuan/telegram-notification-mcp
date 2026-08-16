/**
 * Manual type definitions for Cloudflare Worker environment.
 *
 * Since `--include-env=false` is used with wrangler types, we need to
 * manually define all environment bindings here. This ensures consistent
 * types across local development and CI builds.
 *
 * Secrets (BOT_TOKEN, DEFAULT_CHAT_ID, MCP_AUTH_TOKEN) are configured in the
 * Cloudflare dashboard and read from .dev.vars locally.
 *
 * @see https://github.com/cloudflare/workers-sdk/issues/5756
 */

// Augment the Cloudflare namespace with our env bindings
declare global {
	namespace Cloudflare {
		interface Env {
			/** Telegram Bot API token from @BotFather */
			BOT_TOKEN: string;
			/** Default chat ID for sending notifications (optional) */
			DEFAULT_CHAT_ID?: string;
			/** Shared bearer token required by clients connecting to the MCP endpoint */
			MCP_AUTH_TOKEN: string;
		}
		interface GlobalProps {
			mainModule: typeof import("./index");
		}
	}

	// Top-level Env interface that extends Cloudflare.Env
	interface Env extends Cloudflare.Env {}
}

export {};
