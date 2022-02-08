import type { CreateExercise } from "@/models/Exercise";
import type Exercise from "@/models/Exercise";
import useSWRV, { mutate } from "swrv";
import { prefetchSolutions } from "./solution";

export function prefetchExercises() {
  const url = "/api/exercise";
  mutate(
    url,
    fetch(url).then((r) => r.json())
  );
}

export function prefetchExercise(id: number, data?: Exercise) {
  const url = `/api/exercise/${id}`;
  mutate(url, data ?? fetch(url).then((r) => r.json()));
}

export function useExercises() {
  const { data, error } = useSWRV<Exercise[]>("/api/exercise");
  data.value?.forEach((exercise) => {
    prefetchExercise(exercise.ex_id);
    prefetchSolutions(exercise.ex_id);
  });
  return { data, error };
}

export function useExercise(id: number) {
  const { data, error } = useSWRV<Exercise>(`/api/exercise/${id}`);
  return { data, error };
}

export function likeExercise(exercise: Exercise): Promise<void> {
  const url = `/api/exercise/${exercise.ex_id}/like`;
  const resp = fetch(url, { method: "POST" });
  prefetchExercise(exercise.ex_id, {
    ...exercise,
    ex_liked_by_me: true,
    ex_likes: exercise.ex_likes + 1,
  });
  prefetchExercise(exercise.ex_id);
  return resp.then(() => undefined);
}

export function unlikeExercise(exercise: Exercise) {
  const url = `/api/exercise/${exercise.ex_id}/unlike`;
  const resp = fetch(url, { method: "POST" });
  prefetchExercise(exercise.ex_id, {
    ...exercise,
    ex_liked_by_me: false,
    ex_likes: exercise.ex_likes - 1,
  });
  prefetchExercise(exercise.ex_id);
  return resp.then(() => undefined);
}

export async function createExercise(
  exercise: CreateExercise
): Promise<number> {
  const url = "/api/exercise";
  const resp = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(exercise),
  });
  if (!resp.ok) {
    throw new Error(await resp.text());
  }
  const id = ((await resp.json()) as Exercise).ex_id;
  prefetchExercise(id);
  return id;
}
