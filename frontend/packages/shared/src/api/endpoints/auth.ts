import { ApiClient } from "../client";
import type { LoginRequest, LoginResponse, User } from "../../types";

// Stub — returns empty structures. Real implementation in future issues.

export function createAuthEndpoints(client: ApiClient) {
  return {
    login: (req: LoginRequest) =>
      client.post<LoginResponse>("/auth/login", req),

    logout: () =>
      client.post<void>("/auth/logout"),

    me: () =>
      client.get<User>("/auth/me"),

    changePassword: (current: string, newPassword: string) =>
      client.put<void>("/auth/password", { current_password: current, new_password: newPassword }),
  };
}

export type AuthEndpoints = ReturnType<typeof createAuthEndpoints>;
