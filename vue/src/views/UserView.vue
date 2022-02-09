<script setup lang="ts">
import { useUser } from "@/api/user";
import { useRoute } from "vue-router";
import ErrorCard from "../components/ErrorCard.vue";

const $route = useRoute();
const username = $route.params.username as string;

const { data: user, error } = useUser(username);
</script>

<template>
  <main>
    <template v-if="user">
      <typewriter-heading>USER: {{ user.u_name }}</typewriter-heading>
      <hr />
      <dl class="user-dl --typewriter">
        <dt>Username</dt>
        <dd>{{ user.u_name }}</dd>
        <dt>Email</dt>
        <dd>{{ user.u_email }}</dd>
        <dt>Permission</dt>
        <dd>{{ user.u_permission }}</dd>
        <dt>Score</dt>
        <dd>{{ user.u_score }}</dd>
        <dt>Created</dt>
        <dd>{{ user.u_created_at.replace("T", "") }}</dd>
      </dl>
    </template>
    <loading-card v-if="user === undefined && !error" />
    <ErrorCard v-if="error">Failed to load exercises: {{ error }}</ErrorCard>
  </main>
</template>

<style scoped>
@import "@/assets/animations.css";

.user-dl dt {
  font-weight: bold;
}

.user-dl dt::before {
  content: "➜ ";
}

.user-dl dd {
  margin-bottom: 1em;
}
</style>
