import { Env } from '../index';

interface PushNotification {
  userId: string;
  title: string;
  body: string;
  priority: 'high' | 'normal' | 'low';
}

export async function sendPushNotification(env: Env, notification: PushNotification) {
  const userTokens = await env.KV.get(`push_tokens:${notification.userId}`, 'json') as any;

  if (!userTokens) {
    console.log(`No push tokens found for user ${notification.userId}`);
    return;
  }

  const promises = [];

  if (userTokens.apns) {
    promises.push(sendAPNS(env, userTokens.apns, notification));
  }

  if (userTokens.fcm) {
    promises.push(sendFCM(env, userTokens.fcm, notification));
  }

  await Promise.allSettled(promises);
}

async function sendAPNS(env: Env, token: string, notification: PushNotification) {
  console.log(`Sending APNS notification to ${token}`);
}

async function sendFCM(env: Env, token: string, notification: PushNotification) {
  const message = {
    to: token,
    notification: {
      title: notification.title,
      body: notification.body,
      priority: notification.priority,
    },
    data: {
      userId: notification.userId,
      timestamp: new Date().toISOString(),
    },
  };

  const response = await fetch('https://fcm.googleapis.com/fcm/send', {
    method: 'POST',
    headers: {
      'Authorization': `key=${env.PUSH_KEY}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(message),
  });

  if (!response.ok) {
    throw new Error(`FCM error: ${response.status}`);
  }
}