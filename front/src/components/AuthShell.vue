<script setup>
import AppLogo from '@/components/AppLogo.vue'
</script>

<template>
  <div class="auth-page">
    <main class="auth-frame">
      <section class="brand-panel">
        <AppLogo tone="light" />

        <div class="brand-copy">
          <p class="eyebrow">BOOK MANAGER</p>
          <h1>让每一本书<br /><em>找到它的读者</em></h1>
          <p class="lede">
            一个安静、清晰的馆藏空间。借出一本好书，归还一段旅程，让阅读在人与人之间继续流动。
          </p>
        </div>

        <div class="shelf" aria-hidden="true">
          <span class="shelf__book" style="--book-height: 74px; --book-color: #d9a441"></span>
          <span class="shelf__book" style="--book-height: 104px; --book-color: #cf6b4e"></span>
          <span class="shelf__book" style="--book-height: 90px; --book-color: #8aa38c"></span>
          <span class="shelf__book" style="--book-height: 112px; --book-color: #e4d5b2"></span>
          <span class="shelf__book" style="--book-height: 82px; --book-color: #5d8a77"></span>
          <span class="shelf__book" style="--book-height: 100px; --book-color: #b9825b"></span>
        </div>

        <div class="brand-foot">
          <span>READ · BORROW · RETURN</span>
          <span>书海图书管理系统</span>
        </div>
      </section>

      <section class="form-panel">
        <div class="form-panel__inner">
          <slot />
        </div>
      </section>
    </main>
  </div>
</template>

<style scoped>
.auth-page {
  display: grid;
  min-height: 100vh;
  place-items: center;
  padding: clamp(16px, 3vw, 32px);
}

.auth-frame {
  display: grid;
  grid-template-columns: minmax(0, 1.04fr) minmax(0, 0.96fr);
  width: min(1160px, 100%);
  min-height: min(780px, calc(100vh - 64px));
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.72);
  border-radius: clamp(24px, 3vw, 40px);
  background: rgba(255, 253, 248, 0.72);
  box-shadow:
    0 44px 100px -62px rgba(13, 44, 36, 0.65),
    inset 0 1px 0 rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(18px);
  animation: auth-rise 0.72s cubic-bezier(0.2, 0.7, 0.2, 1) both;
}

.brand-panel {
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  gap: 40px;
  overflow: hidden;
  padding: clamp(32px, 4.2vw, 56px);
  color: #f6f0e4;
  background:
    radial-gradient(circle at 82% 0%, rgba(233, 198, 139, 0.22), transparent 34%),
    radial-gradient(circle at 0% 100%, rgba(127, 154, 128, 0.3), transparent 45%),
    linear-gradient(145deg, #17483b 0%, #0f332a 55%, #0a2821 100%);
}

.brand-panel::before {
  position: absolute;
  top: -230px;
  right: -180px;
  width: 460px;
  height: 460px;
  border: 1px solid rgba(246, 240, 228, 0.12);
  border-radius: 50%;
  content: '';
}

.brand-panel::after {
  position: absolute;
  bottom: -170px;
  left: -130px;
  width: 340px;
  height: 340px;
  border: 1px solid rgba(246, 240, 228, 0.1);
  border-radius: 50%;
  content: '';
}

.brand-panel > * {
  position: relative;
  z-index: 1;
}

.brand-copy {
  max-width: 440px;
}

.brand-copy .eyebrow {
  color: rgba(246, 240, 228, 0.52);
}

.brand-copy h1 {
  margin-top: 18px;
  font-family: var(--font-serif);
  font-size: clamp(2.2rem, 3.6vw, 3.35rem);
  font-weight: 600;
  letter-spacing: -0.02em;
  line-height: 1.16;
}

.brand-copy h1 em {
  color: var(--amber-soft);
  font-style: normal;
}

.brand-copy .lede {
  max-width: 370px;
  margin-top: 20px;
  color: rgba(246, 240, 228, 0.64);
  font-size: 0.94rem;
  line-height: 1.8;
}

.shelf {
  position: relative;
  display: flex;
  align-items: flex-end;
  gap: 9px;
  height: 126px;
  padding: 0 4px;
}

.shelf::after {
  position: absolute;
  right: 0;
  bottom: -12px;
  left: 0;
  height: 1px;
  background: linear-gradient(
    90deg,
    rgba(246, 240, 228, 0),
    rgba(246, 240, 228, 0.3),
    rgba(246, 240, 228, 0)
  );
  content: '';
}

.shelf__book {
  position: relative;
  display: block;
  width: clamp(16px, 1.8vw, 22px);
  height: var(--book-height);
  border-radius: 3px 6px 6px 3px;
  background: linear-gradient(90deg, rgba(0, 0, 0, 0.2), transparent 32%), var(--book-color);
  box-shadow:
    inset -2px 0 rgba(0, 0, 0, 0.12),
    0 16px 26px -16px rgba(0, 0, 0, 0.75);
}

.shelf__book::before,
.shelf__book::after {
  position: absolute;
  right: 3px;
  left: 3px;
  height: 1px;
  background: rgba(255, 255, 255, 0.3);
  content: '';
}

.shelf__book::before {
  top: 12px;
}

.shelf__book::after {
  bottom: 12px;
}

.shelf__book:nth-child(3) {
  transform: translateY(1px) rotate(-8deg);
  transform-origin: bottom right;
}

.shelf__book:nth-child(4) {
  margin-left: 5px;
}

.brand-foot {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  color: rgba(246, 240, 228, 0.4);
  font-size: 0.66rem;
  letter-spacing: 0.17em;
  text-transform: uppercase;
}

.form-panel {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: clamp(28px, 4.5vw, 64px);
  background: linear-gradient(180deg, rgba(255, 253, 248, 0.5), rgba(255, 253, 248, 0.88));
}

.form-panel__inner {
  width: min(400px, 100%);
}

@media (max-width: 900px) {
  .auth-frame {
    grid-template-columns: 1fr;
    min-height: auto;
  }

  .brand-panel {
    gap: 24px;
    padding: 28px 30px 32px;
  }

  .brand-copy h1 {
    font-size: clamp(1.8rem, 6vw, 2.4rem);
  }

  .brand-copy .lede,
  .shelf,
  .brand-foot {
    display: none;
  }

  .form-panel {
    padding: 36px 30px 42px;
  }
}
</style>
