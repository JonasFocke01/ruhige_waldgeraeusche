<script>
export default {
  props: {},
  data() {
    return {
      value: 70,
    };
  },
  methods: {},
};
</script>

<template>
  <input
    class="rounded-lg m-2 px-2 py-1 font-bold text-gray-100 border-2 border-black"
    name="im_a_range"
    type="range"
    min="0"
    max="255"
    v-model.lazy="value"
  />
  {{ value }}
</template>
