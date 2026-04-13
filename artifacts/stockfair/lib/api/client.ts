import { API_BASE_URL, API_TIMEOUT_MS } from '@/constants/api';
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
