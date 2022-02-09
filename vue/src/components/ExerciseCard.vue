<script setup lang="ts">
import type Exercise from "@/models/Exercise";
import { prefetchExercise, likeExercise, unlikeExercise } from "@/api/exercise";
import { ref, toRefs } from "vue";
import { useLogin } from "@/api/login";
import {
  prefetchSolutions,
  useSolutions,
  submitSolution,
} from "@/api/solution";

const props = defineProps({
  exercise: {
    type: Object as () => Exercise,
    required: true,
  },
  standalone: {
    type: Boolean as () => boolean,
    default: false,
  },
});

const { exercise, standalone } = toRefs(props);

const { data: loginClaim } = useLogin();

const { data: solutions } = useSolutions(exercise.value.ex_id);

const solution = ref("");

async function toggleLike() {
  if (!loginClaim.value) return;
  let a = JSON.parse(JSON.stringify(exercise.value));
  // console.log(exercise.value.Target.ex_liked_by_me);
  if (a.liked_by_me) {
    await unlikeExercise(exercise.value);
  } else {
    await likeExercise(exercise.value);
  }
}

async function solve() {
  if (!loginClaim.value) return;
  if (solution.value.length === 0) return;
  await submitSolution(exercise.value, solution.value);
  solution.value = "";
  prefetchSolutions(exercise.value.ex_id);
}
</script>

<template>
  <div class="exercise-card">
    <header class="exercise-title">
      <slot name="title" :title="exercise.ex_name">
        {{ exercise.ex_name }}
      </slot>
    </header>
    <span class="exercise-description">
      <slot name="description" :description="exercise.ex_description">
        {{ exercise.ex_description }}
      </slot>
    </span>
    <h3 v-if="!standalone && exercise.ex_solved_by_me" class="--solved">
      Solved
    </h3>
    <template v-if="standalone">
      <form
        @submit.prevent="solve"
        v-if="!exercise.ex_solved_by_me && loginClaim"
      >
      <!-- button to redirect to the /api/id/input endpoint -->
        <label for="solution">Solution: </label>
        <!-- eslint-disable-next-line prettier/prettier -->
        <input id="solution" type="text" v-model="solution" /> <input type="submit" value="Solve" />
      </form>
      <h3 v-if="exercise.ex_solved_by_me" class="--solved">Solved!</h3>
      <p v-if="!loginClaim" class="exercise-not-logged-in">
        You need to be
        <RouterLink to="/login">logged in</RouterLink>
        to solve exercises
      </p>
      <RouterLink
        class="button"
        :to="`/api/exercise/${exercise.ex_id}/input`"
        > Get your input
        
        </RouterLink>
      <details v-if="loginClaim">
        <summary>Your solutions</summary>
        <ul v-if="solutions">
          <li v-for="attempt in solutions" :key="attempt.s_id">
            <span v-if="attempt.s_correct" class="green">✓</span>
            <span v-else class="red">✗</span>
            {{ attempt.s_answer }}
          </li>
        </ul>
      </details>
    </template>
    <hr />
    <footer class="exercise-footer">
      <a
        id="likeUnlike"
        class="exercise-likes"
        :class="exercise.ex_liked_by_me && '--liked'"
        @mouseover="() => prefetchExercise(exercise.ex_id)"
        @click="toggleLike"
      >
        {{ exercise.ex_likes }}
      </a>
      <!-- eslint-disable-next-line prettier/prettier -->
      <span class="exercise-difficulty">Difficulty: {{ exercise.ex_difficulty }}</span>
    </footer>
  </div>
</template>

<style scoped>

.exercise-description {
  white-space: pre-wrap;
}
.exercise-title {
  font-weight: bold;
  font-size: 1.1em;
  margin-bottom: 0.5rem;
}

.exercise-likes,
.exercise-difficulty {
  font-weight: bold;
  color: var(--c0-c-secondary);
}

.exercise-likes:hover,
.exercise-difficulty:hover {
  text-decoration: underline;
  color: var(--c0-c-tertiary);
}
.exercise-likes::before {
  content: "❤️ ";
}

.--liked,
.--liked:hover {
  color: var(--c0-c-like);
}

.exercise-footer *:first-child {
  margin-right: 0.5rem;
}

.exercise-footer *:last-child {
  margin-left: 0.5rem;
}

.exercise-footer *:not(:first-child):not(:last-child) {
  margin-left: 0.5rem;
  margin-right: 0.5rem;
}

.--solved::before {
  content: "✓ ";
}

.red {
  color: var(--c0-c-red);
}

.green {
  color: var(--c0-c-green);
}

.exercise-not-logged-in {
  text-decoration: underline;
}
</style>
