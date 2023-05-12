<script setup lang="ts">
import { invoke } from '@tauri-apps/api'

import BoxGame from './components/BoxGame.vue';

addEventListener("keydown", async ({ key }) => {  
  const data = await invoke<string>('greet', { key });
});
</script>

<template>
  <div class="container">
    <div class="results-container">
      <!-- Title -->
      <h1 style="font-size: 80px;">
        2048
      </h1>
      
      <!-- Score -->
      <div class="score-container">
        <div
          class="score"
          style="margin-right: 8px;"
        >
          <p style="font-size: 13px">
            SCORE
          </p>
          {{ 428 }}
        </div>
  
        <!-- Best Score -->
        <div class="score">
          <p style="font-size: 13px">
            BEST
          </p>
          {{ 1476 }}
        </div>
      </div>
    </div>

    <!-- New game -->
    <div class="new-game-container">
      <div>
        <p>Play 2048 Game Online</p>
        <p>Join the numbers and get to the <strong>2048 tile!</strong></p>
      </div>

      <button
        type="button"
        class="btn-new-game"
      >
        New Game
      </button>
    </div>

    <div class="game-container">
      <div class="grid-container">
        <div class="grid-row">
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
        </div>

        <div class="grid-row">
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
        </div>

        <div class="grid-row">
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
        </div>

        <div class="grid-row">
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
          <BoxGame :value="'2'" />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
.results-container {
  width: 450px;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
}
.score-container {
  display: flex;
  flex-direction: row;
}
.score {
  background-color: #42B883;
  font-size: 25px;
  font-weight: 700;
  text-align: center;
  padding: 0 15px;
  width: auto;
  border-radius: 5px;
}
.new-game-container {
  width: 450px;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  padding-bottom: 20px;
}
.btn-new-game {
  padding: 5px 10px;
  background-color: #42D392;
  color: #213547;
  border-color: transparent;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.1s;
}
.btn-new-game:active {
  background-color: #2f9969;
}
.game-container {
  background-color: #242424;
  height: 450px;
  width: 450px;
  padding: 12px;
  border-radius: 6px;
}

.grid-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
}
.grid-row {
  width: 100%;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
}
</style>
