<template>
  <Controller_Gui />
</template>

<script>
import Controller_Gui from './components/Controller-Gui.vue';

export default {
  name: 'App',
  components: {
    Controller_Gui,
  },
};
</script>

<style>
@tailwind base;
@tailwind components;
@tailwind utilities;

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
