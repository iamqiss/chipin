// Motherlode API base URL
// In development: Codespaces forwarded port
// In production: deployed motherlode URL
export const API_BASE_URL =
  process.env.EXPO_PUBLIC_API_URL ?? 'http://localhost:8080';

export const API_TIMEOUT_MS = 10_000;

export const ENDPOINTS = {
  // Auth
  AUTH_HEALTH:           '/auth/health',
  AUTH_REGISTER_STEP1:   '/auth/register/step1',
  AUTH_REGISTER_STEP2:   '/auth/register/step2',
  AUTH_REGISTER_STEP3:   '/auth/register/step3',
  AUTH_REGISTER_STEP4:   '/auth/register/step4',
  AUTH_OTP_SEND:         '/auth/otp/send',
  AUTH_OTP_VERIFY:       '/auth/otp/verify',
  AUTH_SIGNIN:           '/auth/signin',
  AUTH_SIGNOUT:          '/auth/signout',
  AUTH_REFRESH:          '/auth/refresh',
  AUTH_FORGOT_PASSWORD:  '/auth/forgot-password',
  AUTH_RESET_PASSWORD:   '/auth/reset-password',
  AUTH_ME:               '/auth/me',
  AUTH_SESSIONS:         '/auth/sessions',
} as const;
