<script>
export default {
  props: {},
  data() {
    return {
      red: 60,
      green: 60,
      blue: 60,
    };
  },
  methods: {
    change_color() {
      //   console.log('test');
      fetch('http://192.168.2.17:5000/change_colors/', {
        method: 'Post',
        body: JSON.stringify({
          red: this.red,
          green: this.green,
          blue: this.blue,
        }),
        headers: { 'Content-Type': 'application/json' },
      });
      this.$emit('change');
    },
  },
};
</script>

<template>
  <div class="flex flex-row mt-20">
    <input
      class="rounded-lg appearance-none bg-red-600 border-2 -rotate-90 -mr-32"
      name="im_a_range"
      type="range"
      min="0"
      max="255"
      v-model="red"
      v-on:input="change_color"
    />

    <input
      class="rounded-lg appearance-none bg-green-600 border-2 -rotate-90 -mr-32"
      name="im_a_range"
      type="range"
      min="0"
      max="255"
      v-model="green"
      v-on:input="change_color"
    />

    <input
      class="rounded-lg appearance-none bg-blue-600 border-2 -rotate-90"
      name="im_a_range"
      type="range"
      min="0"
      max="255"
      v-model="blue"
      v-on:input="change_color"
    />
  </div>
</template>
