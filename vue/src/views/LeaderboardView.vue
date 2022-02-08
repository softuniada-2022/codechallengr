<script setup lang="ts">
import { useScores } from "@/api/score";
import ErrorCard from "../components/ErrorCard.vue";

const { data: scores, error } = useScores();
</script>

<template>
  <typewriter-heading>LEADERBOARD</typewriter-heading>
  <hr />
  <main>
    <ul v-if="scores">
      <li class="leaderboard-entry" v-for="score in scores" :key="score.user">
        <RouterLink :to="`/user/${score.user}`">{{ score.user }}</RouterLink>
        ({{ score.score }} score)
      </li>
    </ul>
    <loading-card v-if="scores === undefined && !error" />
    <ErrorCard v-if="error">Failed to load leaderboard: {{ error }}</ErrorCard>
  </main>
</template>

<style scoped>
.leaderboard-entry {
  /* numbered */
  list-style-type: decimal;
}
</style>
