#!/usr/bin/env python3
"""
StockFair API Client Scaffold
Generates the full API layer for connecting to motherlode.
Run from: StockFair/artifacts/stockfair/
"""

import os

ROOT = "."

FILES = [

# ── Config ────────────────────────────────────────────────────────────────────

(
"constants/api.ts",
"""// Motherlode API base URL
// In development: your Codespaces forwarded port
// In production: your deployed motherlode URL
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
""",
),

# ── Core client ───────────────────────────────────────────────────────────────

(
"lib/api/client.ts",
"""import { API_BASE_URL, API_TIMEOUT_MS } from '@/constants/api';
import { getTokens, clearTokens, saveTokens } from '@/lib/auth/tokens';

export type ApiResponse<T> =
  | { ok: true; data: T }
  | { ok: false; error: string; status: number };

// ── Core fetch wrapper ────────────────────────────────────────────────────────

async function request<T>(
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH',
  endpoint: string,
  body?: object,
  requiresAuth = false,
): Promise<ApiResponse<T>> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), API_TIMEOUT_MS);

  try {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };

    if (requiresAuth) {
      const tokens = await getTokens();
      if (tokens?.accessToken) {
        headers['Authorization'] = `Bearer ${tokens.accessToken}`;
      }
    }

    const res = await fetch(`${API_BASE_URL}${endpoint}`, {
      method,
      headers,
      body: body ? JSON.stringify(body) : undefined,
      signal: controller.signal,
    });

    const data = await res.json();

    if (!res.ok) {
      return {
        ok: false,
        error: data?.error ?? 'Something went wrong',
        status: res.status,
      };
    }

    return { ok: true, data: data as T };
  } catch (err: any) {
    if (err?.name === 'AbortError') {
      return { ok: false, error: 'Request timed out', status: 408 };
    }
    return { ok: false, error: 'Network error. Check your connection.', status: 0 };
  } finally {
    clearTimeout(timeout);
  }
}

// ── Auto-refresh wrapper ──────────────────────────────────────────────────────
// If a protected request returns 401, try refreshing the token once then retry.

async function authRequest<T>(
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH',
  endpoint: string,
  body?: object,
): Promise<ApiResponse<T>> {
  const result = await request<T>(method, endpoint, body, true);

  if (!result.ok && result.status === 401) {
    // Try to refresh
    const tokens = await getTokens();
    if (!tokens?.refreshToken) {
      await clearTokens();
      return result;
    }

    const refreshResult = await request<{ access_token: string; refresh_token: string }>(
      'POST',
      '/auth/refresh',
      { refresh_token: tokens.refreshToken },
      false,
    );

    if (!refreshResult.ok) {
      await clearTokens();
      return { ok: false, error: 'Session expired. Please sign in again.', status: 401 };
    }

    await saveTokens({
      accessToken: refreshResult.data.access_token,
      refreshToken: refreshResult.data.refresh_token,
    });

    // Retry original request with new token
    return request<T>(method, endpoint, body, true);
  }

  return result;
}

// ── Exported API methods ──────────────────────────────────────────────────────

export const api = {
  get:    <T>(endpoint: string) => authRequest<T>('GET', endpoint),
  post:   <T>(endpoint: string, body?: object) => authRequest<T>('POST', endpoint, body),
  put:    <T>(endpoint: string, body?: object) => authRequest<T>('PUT', endpoint, body),
  delete: <T>(endpoint: string) => authRequest<T>('DELETE', endpoint),
  patch:  <T>(endpoint: string, body?: object) => authRequest<T>('PATCH', endpoint, body),
  // Public endpoints (no auth, no auto-refresh)
  public: {
    get:  <T>(endpoint: string) => request<T>('GET', endpoint, undefined, false),
    post: <T>(endpoint: string, body?: object) => request<T>('POST', endpoint, body, false),
  },
};
""",
),

# ── Token storage ─────────────────────────────────────────────────────────────

(
"lib/auth/tokens.ts",
"""import AsyncStorage from '@react-native-async-storage/async-storage';

const ACCESS_TOKEN_KEY  = '@stockfair:access_token';
const REFRESH_TOKEN_KEY = '@stockfair:refresh_token';

export interface AuthTokens {
  accessToken: string;
  refreshToken: string;
}

export async function saveTokens(tokens: AuthTokens): Promise<void> {
  await AsyncStorage.multiSet([
    [ACCESS_TOKEN_KEY,  tokens.accessToken],
    [REFRESH_TOKEN_KEY, tokens.refreshToken],
  ]);
}

export async function getTokens(): Promise<AuthTokens | null> {
  const pairs = await AsyncStorage.multiGet([ACCESS_TOKEN_KEY, REFRESH_TOKEN_KEY]);
  const accessToken  = pairs[0][1];
  const refreshToken = pairs[1][1];
  if (!accessToken || !refreshToken) return null;
  return { accessToken, refreshToken };
}

export async function clearTokens(): Promise<void> {
  await AsyncStorage.multiRemove([ACCESS_TOKEN_KEY, REFRESH_TOKEN_KEY]);
}

export async function getAccessToken(): Promise<string | null> {
  return AsyncStorage.getItem(ACCESS_TOKEN_KEY);
}
""",
),

# ── Auth types ────────────────────────────────────────────────────────────────

(
"lib/auth/types.ts",
"""// Mirrors src/models/user.rs on the motherlode side

export type SupportedLanguage =
  | 'en' | 'zu' | 'xh' | 'af'
  | 'nso' | 'tn' | 'st' | 'ts'
  | 'ss' | 've' | 'nr';

export type AppTheme = 'obsidian' | 'forge' | 'bloom';

export type Gender =
  | 'male' | 'female'
  | 'non_binary' | 'prefer_not_to_say';

export type StokvelInterest =
  | 'rotation' | 'burial'
  | 'investment' | 'grocery' | 'social';

export type OtpPurpose =
  | 'register' | 'reset_password' | 'withdraw';

// ── User ──────────────────────────────────────────────────────────────────────

export interface UserProfile {
  id: string;
  phone: string;
  email: string | null;
  full_name: string;
  avatar_url: string | null;
  language: SupportedLanguage;
  theme: AppTheme;
  is_kyc_verified: boolean;
  created_at: string;
}

// ── Registration steps ────────────────────────────────────────────────────────

export interface RegisterStep1Payload {
  first_name: string;
  last_name: string;
  date_of_birth: string; // DD/MM/YYYY
  gender: Gender;
}

export interface RegisterStep2Payload {
  session_key: string;
  phone: string;
  email?: string;
  password: string;
  confirm_password: string;
}

export interface RegisterStep3Payload {
  phone: string;
  language: SupportedLanguage;
}

export interface RegisterStep4Payload {
  phone: string;
  interests: StokvelInterest[];
  theme?: AppTheme;
  terms_accepted: boolean;
}

// ── OTP ───────────────────────────────────────────────────────────────────────

export interface SendOtpPayload {
  phone: string;
  purpose: OtpPurpose;
}

export interface VerifyOtpPayload {
  phone: string;
  purpose: OtpPurpose;
  code: string;
}

// ── Sign in ───────────────────────────────────────────────────────────────────

export interface SignInPayload {
  identifier: string; // phone or email
  password: string;
}

// ── Password reset ────────────────────────────────────────────────────────────

export interface ForgotPasswordPayload {
  phone: string;
}

export interface ResetPasswordPayload {
  phone: string;
  otp_token: string;
  new_password: string;
  confirm_new_password: string;
}

// ── Responses ─────────────────────────────────────────────────────────────────

export interface AuthResponse {
  access_token: string;
  refresh_token: string;
  user: UserProfile;
}

export interface OtpSentResponse {
  message: string;
  expires_in_seconds: number;
  debug_otp?: string; // only in development
}

export interface OtpVerifiedResponse {
  otp_token: string;
  message: string;
}

export interface MessageResponse {
  message: string;
}

export interface Step1Response {
  session_key: string;
  message: string;
}

export interface Step2Response {
  phone: string;
  message: string;
}
""",
),

# ── Auth API ──────────────────────────────────────────────────────────────────

(
"lib/auth/api.ts",
"""import { api } from '@/lib/api/client';
import { ENDPOINTS } from '@/constants/api';
import type {
  AuthResponse,
  ForgotPasswordPayload,
  MessageResponse,
  OtpSentResponse,
  OtpVerifiedResponse,
  RegisterStep1Payload,
  RegisterStep2Payload,
  RegisterStep3Payload,
  RegisterStep4Payload,
  ResetPasswordPayload,
  SendOtpPayload,
  SignInPayload,
  Step1Response,
  Step2Response,
  VerifyOtpPayload,
} from './types';

// ── Registration ──────────────────────────────────────────────────────────────

export const registerStep1 = (payload: RegisterStep1Payload) =>
  api.public.post<Step1Response>(ENDPOINTS.AUTH_REGISTER_STEP1, payload);

export const registerStep2 = (payload: RegisterStep2Payload) =>
  api.public.post<Step2Response>(ENDPOINTS.AUTH_REGISTER_STEP2, payload);

export const registerStep3 = (payload: RegisterStep3Payload) =>
  api.public.post<MessageResponse>(ENDPOINTS.AUTH_REGISTER_STEP3, payload);

export const registerStep4 = (payload: RegisterStep4Payload) =>
  api.public.post<AuthResponse>(ENDPOINTS.AUTH_REGISTER_STEP4, payload);

// ── OTP ───────────────────────────────────────────────────────────────────────

export const sendOtp = (payload: SendOtpPayload) =>
  api.public.post<OtpSentResponse>(ENDPOINTS.AUTH_OTP_SEND, payload);

export const verifyOtp = (payload: VerifyOtpPayload) =>
  api.public.post<OtpVerifiedResponse>(ENDPOINTS.AUTH_OTP_VERIFY, payload);

// ── Sign in / out ─────────────────────────────────────────────────────────────

export const signIn = (payload: SignInPayload) =>
  api.public.post<AuthResponse>(ENDPOINTS.AUTH_SIGNIN, payload);

export const signOut = (refreshToken: string) =>
  api.post<MessageResponse>(ENDPOINTS.AUTH_SIGNOUT, { refresh_token: refreshToken });

// ── Password reset ────────────────────────────────────────────────────────────

export const forgotPassword = (payload: ForgotPasswordPayload) =>
  api.public.post<OtpSentResponse>(ENDPOINTS.AUTH_FORGOT_PASSWORD, payload);

export const resetPassword = (payload: ResetPasswordPayload) =>
  api.public.post<MessageResponse>(ENDPOINTS.AUTH_RESET_PASSWORD, payload);

// ── Current user ──────────────────────────────────────────────────────────────

export const getMe = () =>
  api.get<{ user: import('./types').UserProfile }>(ENDPOINTS.AUTH_ME);
""",
),

# ── Auth context ──────────────────────────────────────────────────────────────

(
"context/AuthContext.tsx",
"""import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  useCallback,
} from 'react';
import { signIn, signOut, getMe } from '@/lib/auth/api';
import { saveTokens, clearTokens, getTokens } from '@/lib/auth/tokens';
import type { UserProfile, SignInPayload } from '@/lib/auth/types';

interface AuthContextValue {
  user: UserProfile | null;
  isLoading: boolean;
  isAuthenticated: boolean;
  login: (payload: SignInPayload) => Promise<{ ok: boolean; error?: string }>;
  logout: () => Promise<void>;
  refreshUser: () => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser]         = useState<UserProfile | null>(null);
  const [isLoading, setLoading] = useState(true);

  // On mount — check for existing tokens and load user
  useEffect(() => {
    (async () => {
      try {
        const tokens = await getTokens();
        if (tokens) {
          const result = await getMe();
          if (result.ok) setUser(result.data.user);
          else await clearTokens();
        }
      } finally {
        setLoading(false);
      }
    })();
  }, []);

  const login = useCallback(async (payload: SignInPayload) => {
    const result = await signIn(payload);
    if (!result.ok) return { ok: false, error: result.error };
    await saveTokens({
      accessToken:  result.data.access_token,
      refreshToken: result.data.refresh_token,
    });
    setUser(result.data.user);
    return { ok: true };
  }, []);

  const logout = useCallback(async () => {
    const tokens = await getTokens();
    if (tokens?.refreshToken) {
      await signOut(tokens.refreshToken).catch(() => {});
    }
    await clearTokens();
    setUser(null);
  }, []);

  const refreshUser = useCallback(async () => {
    const result = await getMe();
    if (result.ok) setUser(result.data.user);
  }, []);

  return (
    <AuthContext.Provider
      value={{
        user,
        isLoading,
        isAuthenticated: !!user,
        login,
        logout,
        refreshUser,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used inside AuthProvider');
  return ctx;
}
""",
),

# ── Registration context ──────────────────────────────────────────────────────

(
"context/RegistrationContext.tsx",
"""import React, { createContext, useContext, useState } from 'react';
import type {
  Gender,
  StokvelInterest,
  SupportedLanguage,
  AppTheme,
} from '@/lib/auth/types';

// Holds registration state across the 4-step flow
interface RegistrationState {
  // Step 1
  firstName: string;
  lastName: string;
  dateOfBirth: string;
  gender: Gender | null;
  sessionKey: string; // returned from step 1 API

  // Step 2
  phone: string;
  email: string;
  password: string;

  // Step 3
  language: SupportedLanguage;

  // Step 4
  interests: StokvelInterest[];
  theme: AppTheme;
  termsAccepted: boolean;

  // OTP
  otpToken: string; // returned after OTP verification
}

const defaultState: RegistrationState = {
  firstName: '',
  lastName: '',
  dateOfBirth: '',
  gender: null,
  sessionKey: '',
  phone: '',
  email: '',
  password: '',
  language: 'en',
  interests: [],
  theme: 'obsidian',
  termsAccepted: false,
  otpToken: '',
};

interface RegistrationContextValue {
  state: RegistrationState;
  update: (patch: Partial<RegistrationState>) => void;
  reset: () => void;
}

const RegistrationContext = createContext<RegistrationContextValue | null>(null);

export function RegistrationProvider({ children }: { children: React.ReactNode }) {
  const [state, setState] = useState<RegistrationState>(defaultState);

  const update = (patch: Partial<RegistrationState>) =>
    setState(prev => ({ ...prev, ...patch }));

  const reset = () => setState(defaultState);

  return (
    <RegistrationContext.Provider value={{ state, update, reset }}>
      {children}
    </RegistrationContext.Provider>
  );
}

export function useRegistration() {
  const ctx = useContext(RegistrationContext);
  if (!ctx) throw new Error('useRegistration must be used inside RegistrationProvider');
  return ctx;
}
""",
),

# ── Auth hooks ────────────────────────────────────────────────────────────────

(
"hooks/useOtp.ts",
"""import { useState, useRef } from 'react';
import { sendOtp, verifyOtp } from '@/lib/auth/api';
import type { OtpPurpose } from '@/lib/auth/types';

export function useOtp(phone: string, purpose: OtpPurpose) {
  const [isSending,   setIsSending]   = useState(false);
  const [isVerifying, setIsVerifying] = useState(false);
  const [error,       setError]       = useState<string | null>(null);
  const [debugOtp,    setDebugOtp]    = useState<string | null>(null);
  const [countdown,   setCountdown]   = useState(0);
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const startCountdown = (seconds: number) => {
    setCountdown(seconds);
    timerRef.current = setInterval(() => {
      setCountdown(prev => {
        if (prev <= 1) {
          clearInterval(timerRef.current!);
          return 0;
        }
        return prev - 1;
      });
    }, 1000);
  };

  const send = async () => {
    if (countdown > 0) return;
    setIsSending(true);
    setError(null);
    const result = await sendOtp({ phone, purpose });
    setIsSending(false);
    if (!result.ok) {
      setError(result.error);
      return false;
    }
    startCountdown(result.data.expires_in_seconds);
    if (result.data.debug_otp) setDebugOtp(result.data.debug_otp);
    return true;
  };

  const verify = async (code: string): Promise<string | null> => {
    setIsVerifying(true);
    setError(null);
    const result = await verifyOtp({ phone, purpose, code });
    setIsVerifying(false);
    if (!result.ok) {
      setError(result.error);
      return null;
    }
    return result.data.otp_token;
  };

  return {
    send,
    verify,
    isSending,
    isVerifying,
    error,
    debugOtp,
    countdown,
    canResend: countdown === 0 && !isSending,
  };
}
""",
),

# ── .env ──────────────────────────────────────────────────────────────────────

(
".env.example",
"""# Motherlode API URL
# Development: your Codespaces forwarded port URL
# Production: your deployed backend URL
EXPO_PUBLIC_API_URL=https://YOUR-CODESPACE-8080.preview.app.github.dev
""",
),

]

def scaffold():
    for path, content in FILES:
        full_path = os.path.join(ROOT, path)
        os.makedirs(os.path.dirname(full_path), exist_ok=True)
        with open(full_path, "w") as f:
            f.write(content)
        print(f"  ✅ {path}")

    print("""
🎉 StockFair API client scaffolded!

Next steps:
  1. Copy .env.example to .env and set EXPO_PUBLIC_API_URL
     to your Codespaces forwarded port URL

  2. Wrap your root layout with providers in app/_layout.tsx:
     import { AuthProvider } from '@/context/AuthContext';
     import { RegistrationProvider } from '@/context/RegistrationContext';

  3. Use in screens:
     import { useAuth } from '@/context/AuthContext';
     import { useOtp } from '@/hooks/useOtp';
     import * as authApi from '@/lib/auth/api';

  4. Test sign in:
     const { login } = useAuth();
     await login({ identifier: '+27821234567', password: 'yourpassword' });
""")

if __name__ == "__main__":
    scaffold()
