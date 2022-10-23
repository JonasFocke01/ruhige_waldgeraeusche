<script>
export default {
  props: {
    label: {
      type: String,
      default: 'click me',
    },
  },
  data() {
    return {
      sees: 0,
    };
  },
  methods: {
    count() {},
  },
  mounted() {},
  buttonClicked() {},
};
</script>

<template>
  <button
    class="bg-gray-800 rounded-lg m-2 px-2 py-1 font-bold text-gray-100 border-2 border-black"
    name="im_a_button"
    type="button"
  >
    {{ label }}
  </button>
</template>
