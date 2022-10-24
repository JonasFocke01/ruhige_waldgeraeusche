<script>
export default {
  props: {},
  data() {
    return {
      level: 1,
    };
  },
  methods: {
    change_level() {
      fetch('http://192.168.2.17:5000/master_level/', {
        method: 'Post',
        body: JSON.stringify({
          value: this.level
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
      class="rounded-lg appearance-none bg-black border-2 -rotate-90"
      name="im_a_range"
      type="range"
      min="0"
      max="1"
      step="0.01"
      v-model="level"
      v-on:input="change_level"
    />
  </div>
</template>
