<template>
    <VRow>
        <VCol v-for="themeItem in themeItems" :key="themeItem.theme" cols="12" sm="6">
            <VThemeProvider :theme="themeItem.theme">
                <VCard @click="selectTheme(themeItem.theme)" :class="{ selected: themeItem.theme === selectedTheme }">
                    <VCardTitle>
                        <VIcon :color="themeItem.color" :icon="themeItem.icon" />
                        {{ themeItem.title }}
                    </VCardTitle>
                    <VCardSubtitle>
                        {{ themeItem.description }}
                    </VCardSubtitle>
                    <VCardText class="ma-n4">
                        <VSkeletonLoader type="paragraph" boilerplate />
                    </VCardText>
                </VCard>
            </VThemeProvider>
        </VCol>
    </VRow>
</template>

<script setup lang="ts">
const selectedTheme = defineModel<string>({ required: true });

const presentation = usePresentation();
const { t } = useI18n();

const themeItems = computed(() => [
    {
        title: t("setting.presentation.theme.dark"),
        description: t("setting.presentation.theme.dark.description"),
        theme: "dark",
        icon: "mdi-weather-night",
        color: "secondary"
    },
    {
        title: t(`setting.presentation.theme.light`),
        description: t(`setting.presentation.theme.light.description`),
        theme: "light",
        icon: "mdi-weather-sunny",
        color: "success"
    }
]);

const selectTheme = (theme: string) => {
    presentation.applyTheme(theme);
    selectedTheme.value = theme;
};
</script>

<style lang="scss">
.v-card {
    border: 1px solid transparent;
    &.selected {
        border-color: rgb(var(--v-theme-info));
    }
}
</style>
