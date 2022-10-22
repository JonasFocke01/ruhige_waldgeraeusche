<template>
  <div class="text-3xl font-bold bg-green-600 py-2 flex flex-row">
    <p class="w-1/3"></p>

    <p class="w-1/3 underline">Ruhige Waldgeräusche</p>
    <div class="w-1/3">
      <p class="w-3/4 ml-auto text-2xl bg-green-700">
        Klick counter: {{ click_counter }}
      </p>
    </div>
  </div>
  <ButtonComponent v-on:click="spawn_snake" v-bind="propsToPass" />
  <SliderComponent v-on:change="processkeypressed" />
  <div class="bg-green-100">
    {{ sees }}
  </div>
</template>

<script>
import ButtonComponent from '../components/Button-Component.vue';
import SliderComponent from '../components/Slider-Component.vue';

export default {
  components: {
    ButtonComponent,
    SliderComponent,
  },
  data() {
    return {
      click_counter: 0,
      propsToPass: {
        label: 'John',
      },
      sees: 150,
    };
  },
  methods: {
    processkeypressed(e) {
      if (e) {
        if (e.key === 's') {
          this.spawn_snake();
        } else {
          this.click_counter += 1;
        }
      }
    },
    spawn_snake() {
      console.log('snake');
      this.click_counter += 1;
      fetch('http://192.168.2.17:5000/animation_snake/');
    },
  },
  mounted() {
    console.log('test');
    fetch('http://192.168.2.17:5000/');
    document.addEventListener('keydown', this.processkeypressed);
  },
};
</script>
