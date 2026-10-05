function addInputIngredients() {
        const input = document.createElement("input");

        input.type = "text";
        input.name = "ingredients";
        input.placeholder = "es. Carrot";
        input.className = "border-2 border-blue-500 mb-5 mr-5 rounded-sm px-4 py-2 hover:bg-gray-200 transition";
        
        const button = document.getElementById("addIngredient")
        //lets you specify exactly where it goes.
        document.getElementById("ingredients").insertBefore(input, button);
}