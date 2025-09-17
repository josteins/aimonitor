import jwt from '@tsndr/cloudflare-worker-jwt';
import { Env } from '../index';

export async function handleAuth(request: Request, env: Env) {
  const authHeader = request.headers.get('Authorization');

  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    return new Response('Unauthorized', { status: 401 });
  }

  const token = authHeader.substring(7);

  try {
    const isValid = await jwt.verify(token, env.JWT_SECRET);

    if (!isValid) {
      return new Response('Unauthorized', { status: 401 });
    }

    const decoded = jwt.decode(token);
    (request as any).user = decoded.payload;
  } catch (error) {
    return new Response('Invalid token', { status: 401 });
  }
}