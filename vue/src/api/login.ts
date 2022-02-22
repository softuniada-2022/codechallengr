import useSWRV, { mutate } from "swrv";
import * as jose from "jose";
import type Claim from "@/models/Claim";
import { prefetchExercises } from "./exercise";
import { prefetchUsers } from "./user";

export function prefetchLogin(claim?: Claim) {
  const url = "/api/login";
  const claimPromise = claim
    ? Promise.resolve(claim)
    : fetch(url).then(async (r) => {
        const token = await r.text();
        const claim = jose.decodeJwt(token) as unknown as Claim;
        if (claim.exp < Date.now() / 1000) {
          throw new Error("Token expired");
        }
        return claim;
      });
  mutate(url, claimPromise);
  return claimPromise;
}

export async function renewLogin() {
  const url = "/api/login?renew";
  const token = await fetch(url).then((r) => r.text());
  await prefetchLogin(jose.decodeJwt(token) as unknown as Claim);
}

export function useLogin() {
  const { data, error } = useSWRV<Claim>("/api/login", async (url) => {
    const token = await fetch(url).then((r) => r.text());
    const claim = jose.decodeJwt(token) as unknown as Claim;
    if (claim.exp - Date.now() / 1000 < 60 * 60) {
      renewLogin();
    }
    if (claim.exp < Date.now() / 1000) {
      throw new Error("Login session expired");
    }
    return claim;
  });
  return { data, error };
}

export async function logIn(username: string, password: string): Promise<void> {
  const url = "/api/login";
  const body = JSON.stringify({ u_name: username, u_password: password });
  const resp = await fetch(url, {
    method: "POST",
    body,
    headers: {
      "Content-Type": "application/json",
    },
  });
  if (!resp.ok) {
    throw new Error(await resp.text());
  }
  prefetchLogin();
  prefetchExercises();
  prefetchUsers();
}

export async function logOut(): Promise<void> {
  const url = "/api/login";
  const resp = await fetch(url, { method: "DELETE" });
  if (!resp.ok) {
    throw new Error(await resp.text());
  }
  mutate(url, Promise.resolve(null));
  prefetchExercises();
  prefetchUsers();
}
