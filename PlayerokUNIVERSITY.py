import tkinter as tk
from tkinter import ttk, messagebox
import customtkinter as ctk
import json
import os

# Настройки темы: Светлая бело-синяя гамма
ctk.set_appearance_mode("Light")
ctk.set_default_color_theme("blue")


class PlayerokUniversity(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.title("PlayerokUniversity ⚡️")
        self.geometry("1200x800")
        self.configure(fg_color="#f1f5f9")  # Светлый фон (Slate 100)

        # Данные
        self.data_file = "data.json"
        self.items = self.load_data()

        # Переменные
        self.name_var = tk.StringVar()
        self.price_var = tk.StringVar(value="0")
        self.purchase_var = tk.StringVar(value="0")
        self.premium_var = tk.StringVar(value="0")

        # Значения по умолчанию для комиссий
        self.sale_comm = 10
        self.withdraw_comm = 6

        self.setup_ui()
        self.update_table()
        self.update_stats()

    def setup_ui(self):
        # --- ЗАГОЛОВОК ---
        self.header_label = ctk.CTkLabel(
            self, text="PlayerokUNIVERSITY",
            font=ctk.CTkFont(family="Arial Black", size=32),
            text_color="#1e40af"  # Насыщенный синий
        )
        self.header_label.pack(pady=(30, 5))

        self.sub_header = ctk.CTkLabel(
            self, text="Система управления доходами ⚡️",
            font=ctk.CTkFont(size=14, slant="italic"),
            text_color="#64748b"
        )
        self.sub_header.pack(pady=(0, 30))

        # --- КАРТОЧКИ СТАТИСТИКИ ---
        self.stats_frame = ctk.CTkFrame(self, fg_color="transparent")
        self.stats_frame.pack(fill="x", padx=40, pady=10)
        self.stats_frame.grid_columnconfigure((0, 1, 2), weight=1)

        self.card_rev = self.create_stat_card(self.stats_frame, "ОБЩАЯ ВЫРУЧКА", "0 ₽", 0)
        self.card_profit = self.create_stat_card(self.stats_frame, "ЧИСТЫЙ ПРОФИТ", "0 ₽", 1, color="#059669")
        self.card_count = self.create_stat_card(self.stats_frame, "ВСЕГО СДЕЛОК", "0", 2)

        # --- ПАНЕЛЬ ВВОДА ---
        self.input_container = ctk.CTkFrame(self, fg_color="#ffffff", corner_radius=20, border_width=1,
                                            border_color="#e2e8f0")
        self.input_container.pack(fill="x", padx=40, pady=25)

        # Сетка ввода
        # 1. Название
        ctk.CTkLabel(self.input_container, text="Название товара", text_color="#475569",
                     font=ctk.CTkFont(size=12, weight="bold")).grid(row=0, column=0, padx=20, pady=(15, 0), sticky="w")
        self.entry_name = ctk.CTkEntry(self.input_container, textvariable=self.name_var, width=200, fg_color="#f8fafc",
                                       border_color="#cbd5e1", text_color="#1e293b")
        self.entry_name.grid(row=1, column=0, padx=20, pady=(5, 15))

        # 2. Цена
        ctk.CTkLabel(self.input_container, text="Цена сайта", text_color="#475569",
                     font=ctk.CTkFont(size=12, weight="bold")).grid(row=0, column=1, padx=20, pady=(15, 0), sticky="w")
        self.entry_price = ctk.CTkEntry(self.input_container, textvariable=self.price_var, width=100,
                                        fg_color="#f8fafc", border_color="#cbd5e1", text_color="#1e293b")
        self.entry_price.grid(row=1, column=1, padx=20, pady=(5, 15))

        # 3. Выручка %
        ctk.CTkLabel(self.input_container, text="Выручка %", text_color="#475569",
                     font=ctk.CTkFont(size=12, weight="bold")).grid(row=0, column=2, padx=20, pady=(15, 0), sticky="w")
        self.seg_sale = ctk.CTkSegmentedButton(self.input_container, values=["10%", "20%"],
                                               command=self.change_sale_comm, fg_color="#f1f5f9",
                                               selected_color="#3b82f6")
        self.seg_sale.set("10%")
        self.seg_sale.grid(row=1, column=2, padx=20, pady=(5, 15))

        # 4. Вывод %
        ctk.CTkLabel(self.input_container, text="Вывод %", text_color="#475569",
                     font=ctk.CTkFont(size=12, weight="bold")).grid(row=0, column=3, padx=20, pady=(15, 0), sticky="w")
        self.seg_withdraw = ctk.CTkSegmentedButton(self.input_container, values=["6%", "10%"],
                                                   command=self.change_withdraw_comm, fg_color="#f1f5f9",
                                                   selected_color="#f59e0b")
        self.seg_withdraw.set("6%")
        self.seg_withdraw.grid(row=1, column=3, padx=20, pady=(5, 15))

        # 5. Закуп
        ctk.CTkLabel(self.input_container, text="Закуп", text_color="#475569",
                     font=ctk.CTkFont(size=12, weight="bold")).grid(row=0, column=4, padx=20, pady=(15, 0), sticky="w")
        self.entry_purchase = ctk.CTkEntry(self.input_container, textvariable=self.purchase_var, width=100,
                                           fg_color="#f8fafc", border_color="#cbd5e1", text_color="#1e293b")
        self.entry_purchase.grid(row=1, column=4, padx=20, pady=(5, 15))

        # 6. Премиум
        ctk.CTkLabel(self.input_container, text="Премиум", text_color="#475569",
                     font=ctk.CTkFont(size=12, weight="bold")).grid(row=0, column=5, padx=20, pady=(15, 0), sticky="w")
        self.entry_premium = ctk.CTkEntry(self.input_container, textvariable=self.premium_var, width=100,
                                          fg_color="#f8fafc", border_color="#cbd5e1", text_color="#1e293b")
        self.entry_premium.grid(row=1, column=5, padx=20, pady=(5, 15))

        # Кнопка Добавить
        self.add_btn = ctk.CTkButton(
            self.input_container, text="ДОБАВИТЬ",
            fg_color="#1e40af", hover_color="#1e3a8a",
            font=ctk.CTkFont(weight="bold"),
            height=45, command=self.add_item,
            corner_radius=12
        )
        self.add_btn.grid(row=0, column=6, rowspan=2, padx=25, pady=15)

        # --- ТАБЛИЦА ---
        self.table_frame = ctk.CTkFrame(self, fg_color="#ffffff", corner_radius=25, border_width=1,
                                        border_color="#e2e8f0")
        self.table_frame.pack(fill="both", expand=True, padx=40, pady=(0, 20))

        # Стилизация стандартного Treeview под светлую тему
        style = ttk.Style()
        style.theme_use("default")
        style.configure("Treeview",
                        background="#ffffff",
                        foreground="#334155",
                        fieldbackground="#ffffff",
                        borderwidth=0,
                        font=("Segoe UI", 11),
                        rowheight=40)
        style.configure("Treeview.Heading",
                        background="#f8fafc",
                        foreground="#64748b",
                        font=("Segoe UI", 10, "bold"),
                        borderwidth=0)
        style.map("Treeview", background=[('selected', '#dbeafe')], foreground=[('selected', '#1e40af')])

        columns = ("name", "price", "sale_comm", "withdraw_comm", "purchase", "premium", "on_card", "profit")
        self.tree = ttk.Treeview(self.table_frame, columns=columns, show="headings")

        headers = {
            "name": "ТОВАР", "price": "ЦЕНА", "sale_comm": "ВЫРУЧКА %",
            "withdraw_comm": "ВЫВОД %", "purchase": "ЗАКУП",
            "premium": "ПРЕМИУМ", "on_card": "НА КАРТУ", "profit": "ПРОФИТ"
        }

        for col, text in headers.items():
            self.tree.heading(col, text=text)
            self.tree.column(col, width=100, anchor="center")

        self.tree.column("name", width=250, anchor="w")

        self.tree.pack(fill="both", expand=True, padx=15, pady=(15, 5))

        # --- ПАНЕЛЬ УПРАВЛЕНИЯ ТАБЛИЦЕЙ (Кнопка удаления здесь) ---
        self.table_controls = ctk.CTkFrame(self.table_frame, fg_color="transparent")
        self.table_controls.pack(fill="x", padx=15, pady=(0, 15))

        self.del_btn = ctk.CTkButton(
            self.table_controls, text="УДАЛИТЬ ВЫБРАННЫЙ ТОВАР",
            fg_color="#ef4444", hover_color="#dc2626",
            font=ctk.CTkFont(weight="bold"),
            height=35, command=self.delete_item,
            corner_radius=10
        )
        self.del_btn.pack(side="right")

        # Контекстное меню
        self.tree.bind("<Button-3>", self.show_context_menu)
        self.menu = tk.Menu(self, tearoff=0, bg="#ffffff", fg="#334155", borderwidth=1)
        self.menu.add_command(label="Удалить запись", command=self.delete_item)

    def create_stat_card(self, parent, title, value, col, color="#1e40af"):
        card = ctk.CTkFrame(parent, fg_color="#ffffff", height=110, corner_radius=20, border_width=1,
                            border_color="#e2e8f0")
        card.grid(row=0, column=col, padx=15, sticky="nsew")

        title_lbl = ctk.CTkLabel(card, text=title, font=ctk.CTkFont(size=11, weight="bold"), text_color="#94a3b8")
        title_lbl.pack(pady=(20, 0))

        val_lbl = ctk.CTkLabel(card, text=value, font=ctk.CTkFont(size=26, weight="bold"), text_color=color)
        val_lbl.pack(pady=(5, 20))

        return val_lbl

    def change_sale_comm(self, val):
        self.sale_comm = int(val.replace("%", ""))

    def change_withdraw_comm(self, val):
        self.withdraw_comm = int(val.replace("%", ""))

    def calculate_values(self, price, purchase, premium, s_comm, w_comm):
        rev_after_sale = price * (1 - s_comm / 100)
        on_card = rev_after_sale * (1 - w_comm / 100)
        profit = on_card - purchase - premium
        return round(on_card, 2), round(profit, 2)

    def add_item(self):
        try:
            name = self.name_var.get()
            price = float(self.price_var.get() or 0)
            purchase = float(self.purchase_var.get() or 0)
            premium = float(self.premium_var.get() or 0)

            if not name:
                messagebox.showwarning("Ошибка", "Введите название товара")
                return

            on_card, profit = self.calculate_values(price, purchase, premium, self.sale_comm, self.withdraw_comm)

            new_item = {
                "name": name,
                "price": price,
                "sale_comm": self.sale_comm,
                "withdraw_comm": self.withdraw_comm,
                "purchase": purchase,
                "premium": premium,
                "on_card": on_card,
                "profit": profit
            }

            self.items.append(new_item)
            self.save_data()
            self.update_table()
            self.update_stats()

            # Очистка
            self.name_var.set("")
            self.price_var.set("0")
            self.purchase_var.set("0")
            self.premium_var.set("0")

        except ValueError:
            messagebox.showerror("Ошибка", "Вводите только числа")

    def update_table(self):
        for row in self.tree.get_children():
            self.tree.delete(row)

        for item in reversed(self.items):
            self.tree.insert("", "end", values=(
                item["name"], f"{item['price']} ₽", f"{item['sale_comm']}%",
                f"{item['withdraw_comm']}%", f"{item['purchase']} ₽", f"{item['premium']} ₽",
                f"{item['on_card']} ₽", f"{item['profit']} ₽"
            ))

    def update_stats(self):
        total_profit = sum(item["profit"] for item in self.items)
        total_rev = sum(item["price"] * (1 - item["sale_comm"] / 100) for item in self.items)

        self.card_rev.configure(text=f"{total_rev:,.1f} ₽")
        self.card_profit.configure(text=f"+{total_profit:,.1f} ₽")
        self.card_count.configure(text=str(len(self.items)))

    def show_context_menu(self, event):
        item = self.tree.identify_row(event.y)
        if item:
            self.tree.selection_set(item)
            self.menu.post(event.x_root, event.y_root)

    def delete_item(self):
        selected = self.tree.selection()
        if not selected:
            messagebox.showinfo("Инфо", "Сначала выберите строку в таблице для удаления")
            return

        # Индекс в списке данных (учитываем реверс при отображении)
        idx = len(self.items) - 1 - self.tree.index(selected[0])
        del self.items[idx]
        self.save_data()
        self.update_table()
        self.update_stats()

    def load_data(self):
        if os.path.exists(self.data_file):
            try:
                with open(self.data_file, "r", encoding="utf-8") as f:
                    return json.load(f)
            except:
                return []
        return []

    def save_data(self):
        with open(self.data_file, "w", encoding="utf-8") as f:
            json.dump(self.items, f, ensure_ascii=False, indent=4)


if __name__ == "__main__":
    app = PlayerokUniversity()
    app.mainloop()