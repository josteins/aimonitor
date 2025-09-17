import { Router } from 'itty-router';
import { scheduled } from './scheduled';
import { handleProviderPoll } from './handlers/poll';
import { handleAuth } from './middleware/auth';

export interface Env {
  KV: KVNamespace;
  SECRETS: KVNamespace;
  DB: D1Database;
  JWT_SECRET: string;
  PUSH_KEY: string;
}

const router = Router();

router
  .get('/health', () => new Response('OK', { status: 200 }))
  .post('/api/poll/:provider', handleAuth, handleProviderPoll)
  .get('/api/usage/:userId', handleAuth, async (request, env: Env) => {
    const userId = request.params.userId;
    const usage = await env.KV.get(`usage:${userId}`, 'json');
    return new Response(JSON.stringify(usage || {}), {
      headers: { 'Content-Type': 'application/json' },
    });
  })
  .all('*', () => new Response('Not Found', { status: 404 }));

export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext) {
    return router.handle(request, env, ctx);
  },

  async scheduled(event: ScheduledEvent, env: Env, ctx: ExecutionContext) {
    ctx.waitUntil(scheduled(env));
  },
};