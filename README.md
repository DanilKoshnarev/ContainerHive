# ContainerHive

## Опис
ContainerHive - це потужна система для управління контейнерами, яка включає модулі для створення, розгортання, моніторингу та управління контейнерами.

## Структура проекту
Проект розділений на кілька шарів для покращення читабельності та підтримуваності коду:

- **Domain**: Основна бізнес-логіка та правила.
- **Application**: Інтерфейси, юзкейси та реалізації для роботи з даними.
- **Infrastructure**: Реалізація деталей інфраструктури, таких як моделі даних, контролери та утиліти.
- **Presentation**: Візуалізація даних та взаємодія з користувачем.

## Встановлення
1. Клонуйте репозиторій:
    ```bash
    git clone <URL репозитарію>
    ```
2. Перейдіть до каталогу проекту:
    ```bash
    cd container_hive
    ```
3. Встановіть необхідні залежності:
    ```bash
    cargo build
    ```

## Запуск
Для запуску проекту виконайте наступну команду:
```bash
cargo run
```

## Структура каталогів
```plaintext
container_hive/
├── src/
│   ├── domain/
│   │   ├── entities/
│   │   │   ├── Container.rs
│   │   │   └── ContainerConfig.rs
│   │   ├── repositories/
│   │   │   └── ContainerRepository.rs
│   │   ├── services/
│   │   │   └── ContainerService.rs
│   │   └── use_cases/
│   │       └── ManageContainer.rs
│   ├── infrastructure/
│   │   ├── models/
│   │   │   └── ContainerModel.rs
│   │   ├── repositories/
│   │   │   └── ContainerRepositoryImpl.rs
│   │   ├── controllers/
│   │   │   └── ContainerController.rs
│   ├── application/
│       └── main.rs
├── config/
│   └── config.rs
├── Cargo.toml
└── README.md
```

## Опис компонентів
### Domain
- **Container.rs**: Клас сутності контейнера.
- **ContainerConfig.rs**: Клас сутності конфігурації контейнера.
- **ContainerRepository.rs**: Інтерфейс репозиторію контейнерів.

### Application
- **ManageContainer.rs**: Юзкейс для управління контейнерами.
- **ContainerService.rs**: Сервіс для управління контейнерами.

### Infrastructure
- **ContainerModel.rs**: Модель даних контейнера.
- **ContainerRepositoryImpl.rs**: Реалізація репозиторію контейнерів.
- **ContainerController.rs**: Контролер для управління контейнерами.

## Ліцензія
Цей проект ліцензовано під ліцензією MIT. Для отримання додаткової інформації див. файл LICENSE.
