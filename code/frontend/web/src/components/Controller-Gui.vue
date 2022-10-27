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
  <button
    class="m-2 px-2 py-1 font-bold text-gray-100 border-2 border-black"
    :style="{ 'background-color': currentColor }"
    name="im_a_button"
    type="button"
  >
    COLOR
  </button>
  <div class="justify-center flex flex-row w-full">
    <img src="../assets/Keyboard.png" alt="Keyboard" />
  </div>
</template>

<script>
export default {
  components: {},
  data() {
    return {
      currentColorSwitch: true,
      currentColor: '#ffffff',
      click_counter: 0,
      propsToPass: {
        label: 'John',
      },
      clearProps: {
        label: 'clear_strips',
      },
      holdProps: {
        label: 'flash me',
      },
    };
  },
  methods: {
    processkeypressed(e) {
      if (e.defaultPrevented) {
        return; // Do nothing if the event was already processed
      }
      if (e) {
        this.click_counter += 1;
        if (e.key === 'y') {
          this.currentColor = '#ff0000';
          fetch('http://192.168.2.17:5000/change_colors/', {
            method: 'Post',
            body: JSON.stringify({
              red: 250,
              green: 0,
              blue: 0,
            }),
            headers: { 'Content-Type': 'application/json' },
          });
        } else if (e.key === 'x') {
          this.currentColor = '#00ff00';
          fetch('http://192.168.2.17:5000/change_colors/', {
            method: 'Post',
            body: JSON.stringify({
              red: 0,
              green: 250,
              blue: 0,
            }),
            headers: { 'Content-Type': 'application/json' },
          });
        } else if (e.key === 'c') {
          this.currentColor = '#0000ff';
          fetch('http://192.168.2.17:5000/change_colors/', {
            method: 'Post',
            body: JSON.stringify({
              red: 0,
              green: 0,
              blue: 250,
            }),
            headers: { 'Content-Type': 'application/json' },
          });
        } else if (e.key === 'v') {
          this.currentColor = '#fbff21';
          fetch('http://192.168.2.17:5000/change_colors/', {
            method: 'Post',
            body: JSON.stringify({
              red: 251,
              green: 250,
              blue: 33,
            }),
            headers: { 'Content-Type': 'application/json' },
          });
        } else if (e.key === 'b') {
          this.currentColor = '#b34af0';
          fetch('http://192.168.2.17:5000/change_colors/', {
            method: 'Post',
            body: JSON.stringify({
              red: 179,
              green: 74,
              blue: 240,
            }),
            headers: { 'Content-Type': 'application/json' },
          });
        } else if (e.key === 'n') {
          this.currentColor = '#4af0ea';
          fetch('http://192.168.2.17:5000/change_colors/', {
            method: 'Post',
            body: JSON.stringify({
              red: 74,
              green: 240,
              blue: 234,
            }),
            headers: { 'Content-Type': 'application/json' },
          });
        } else if (e.key === 'm') {
          this.currentColor = '#f04a8c';
          fetch('http://192.168.2.17:5000/change_colors/', {
            method: 'Post',
            body: JSON.stringify({
              red: 240,
              green: 74,
              blue: 140,
            }),
            headers: { 'Content-Type': 'application/json' },
          });
        } else if (e.key === ',') {
          this.currentColor = '#f0844a';
          fetch('http://192.168.2.17:5000/change_colors/', {
            method: 'Post',
            body: JSON.stringify({
              red: 240,
              green: 132,
              blue: 74,
            }),
            headers: { 'Content-Type': 'application/json' },
          });
        } else if (e.key === '.') {
          this.currentColor = '#4ab3f0';
          fetch('http://192.168.2.17:5000/change_colors/', {
            method: 'Post',
            body: JSON.stringify({
              red: 74,
              green: 179,
              blue: 240,
            }),
            headers: { 'Content-Type': 'application/json' },
          });
        } else if (e.key === 's') {
          fetch('http://192.168.2.17:5000/animation_snake/');
        } else if (e.key === 'w') {
          fetch('http://192.168.2.17:5000/animation_pitch/');
        } else if (e.key === 'a') {
          fetch('http://192.168.2.17:5000/animation_shifting_blocks/');
        } else if (e.key === 'd') {
          fetch('http://192.168.2.17:5000/animation_rain_drops/');
        } else if (e.key === ' ') {
          fetch('http://192.168.2.17:5000/short_flash/');
        } else if (e.key === 'Enter') {
          fetch('http://192.168.2.17:5000/flash_fade/');
        } else if (e.key === 'Control') {
          if (this.currentColorSwitch) {
            this.currentColorSwitch = !this.currentColorSwitch;
            fetch('http://192.168.2.17:5000/color_mode_inherit/');
          } else {
            this.currentColorSwitch = !this.currentColorSwitch;
            fetch('http://192.168.2.17:5000/color_mode_white/');
          }
        } else {
          console.log(e.key);
        }
      }
      e.preventDefault();
    },
  },
  mounted() {
    fetch('http://192.168.2.17:5000/');
    document.addEventListener('keydown', this.processkeypressed);
  },
};
</script>
