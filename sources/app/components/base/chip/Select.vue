<template>
    <div class="v-chip-group">
        <!-- Index used here as key, because it is allowed to pass null as itemValue-->
        <template v-for="(item, index) in items" :key="index">
            <VChip @click="itemSelected(item)" :color="isSelected(item) ? 'primary' : ''" :variant="isSelected(item) ? 'flat' : 'tonal'" density="comfortable">
                {{ item[itemText] }}
            </VChip>
        </template>
    </div>
</template>

<script setup lang="ts" generic="T">
const emit = defineEmits<{
    itemSelected: [item: T];
}>();

const props = defineProps<{
    items: T[];
    itemText: keyof T;
    itemValue: keyof T;
    returnObject?: boolean;
}>();

const selectedItem = defineModel<unknown>();

const itemSelected = (item: T) => {
    selectedItem.value = props.returnObject ? item : item[props.itemValue];
    emit("itemSelected", item);
};

const isSelected = (item: T) => {
    if (props.returnObject) {
        return selectedItem.value === item;
    } else {
        return selectedItem.value === item[props.itemValue];
    }
};
</script>
