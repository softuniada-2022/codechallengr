import type Score from "@/models/Score";
import useSWRV, { mutate } from "swrv";

export function prefetchScores() {
  const url = "/api/score";
  mutate(
    url,
    fetch(url).then((r) => r.json())
  );
}

export function useScores() {
  const url = "/api/score";
  const { data, error } = useSWRV<Score[]>(url);
  return { data, error };
}
