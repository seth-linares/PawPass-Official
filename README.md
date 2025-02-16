# PawPass🐾 - Your Trusty Password Companion 😸

PawPass is a *secure*, *offline*, and *open-source* password manager that puts you in control of your digital security rather than relying on the security of paid services to keep your data safe. While they may be convenient, cloud-based password managers are a prime target for hackers, and their security is only as strong as the weakest link in their chain. PawPass is built with Rust and modern web technologies, providing a delightful and secure way to manage your passwords without relying on cloud services or premium subscriptions.

## Why PawPass? 

PawPass runs entirely on your machine. Rather than having your sensitive data leave your computer, PawPass ensures that everything remains local and encrypted using **MILITARY-GRADE cryptography**. 

One of the best use cases for PawPass is if you work in a secure, offline environment, such as a government agency or a research lab. In these cases, cloud-based password managers are a no-go, and PawPass is the perfect solution. You get to have your cake and eat it too - a beautiful, modern interface with the security of a fortress.

## What's Under the Hood? 

PawPass combines the best of both worlds: incredible performance and rock-solid security and memory-safety from Rust, with a modern, intuitive interface built using React. Here's a quick overview of the technologies we use:

### Backend (The Security Fortress)
- **Rust**: For its uncompromising memory safety and performance
- **Argon2id**: State-of-the-art password hashing for maximum protection
- **AES-GCM**: Military-grade encryption for your sensitive data
- **Tauri**: Secure bridge between our frontend and backend

### Frontend (The Friendly Face)
- **React + TypeScript**: For a type-safe, responsive interface
- **Tailwind CSS (w/ DaisyUI) + shadcn/ui**: Beautiful, accessible components
- **Framer Motion**: Smooth animations that make password management less boring
- **Vite**: Lightning-fast development and optimized builds

## Getting Started

Since I can't afford to pay for a code-signing certificate, you'll need to build PawPass yourself. But don't worry, it's super easy! Here's how you can get started:

1. Clone the repository: `git clone https://github.com/yourusername/PawPass.git`