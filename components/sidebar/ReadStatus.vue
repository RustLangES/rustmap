<template>
  <Dropdown
    ref="statusDropDown"
    :customTriggerClass="
      'px-3 py-1 hover:bg-orange-100 dark:hover:bg-black dark:border-white border' + status.toLowerCase()"
    :border="false"
  >
    <!-- trigger element -->
    <template #trigger>
      <button type="button" v-text="status" />
    </template>

    <!-- contents display in dropdown -->
    <ul class="flex flex-col bg-orange-100 dark:bg-[#131313]">
      <li
        v-for="(s, i) in allStatus"
        :key="'status-' + i"
        @click="changeStatus(s)"
        :class="[
          'px-4 py-2 hover:cursor-pointer border-b border-orange-200 dark:border-gray-500 hover:bg-orange-50 dark:hover:bg-black',
          s.toLowerCase(),
        ]"
        v-text="s"
      ></li>
    </ul>
  </Dropdown>
</template>

<script setup>
import { ref } from "vue";
import Dropdown from "v-dropdown";

const { $locally } = useNuxtApp();
const allStatus = ["Pendiente", "Leyendo", "Completado", "Omitir"];
const statusDropDown = ref(null);

const props = defineProps({
  name: {
    type: String
  },
})

const status = ref($locally.get(props.name) ?? "Pendiente");

function changeStatus(val) {
  if (val == status.value) return;
  if (!statusDropDown.value) return;
  status.value = val;
  $locally.set(props.name, val);
  statusDropDown.value.close();
}
</script>

<style>
.pendiente::before,
.leyendo::before,
.completado::before,
.omitir::before {
  content: "•";
  margin-right: 5px;
}

.pendiente::before {
  color: var(--tw-text-gray-950);
}
.leyendo::before {
  color: var(--tw-text-orange-500);
}
.completado::before {
  color: var(--tw-text-green-600);
}
.omitir::before {
  color: var(--tw-text-slate-500);
}
.v-dropdown-container.v-dropdown-no-border {
  border-radius: 0px !important;
}
</style>
