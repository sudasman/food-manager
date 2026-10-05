function addInputIngredients() {
    const input = document.createElement("input");

    input.type = "text";
    input.name = "ingredients";
    input.placeholder = "es. Carrot";
    input.className = "border-2 border-blue-500 mb-5 mr-5 rounded-sm px-4 py-2 hover:bg-gray-200 transition";
        
    const deleteButton = document.createElement("button");
    deleteButton.type = "button";
    deleteButton.textContent ="-";
    deleteButton.onclick = "deleteInput()";
    deleteButton.className = "border-2 border-red-400  bg-red-400 text-white rounded-sm mr-5 text-xl w-6 h-6 px-2 flex items-center justify-center leading-non hover:text-black hover:border-black transition duration-300";

    function deleteInput(){}

    const div = document.createElement("div");
    div.className = "flex justify-center items-center";

    div.appendChild(input);
    div.appendChild(deleteButton);

    const addButton = document.getElementById("addIngredient")
    //lets you specify exactly where it goes.
    document.getElementById("ingredients").insertBefore(div, addButton);
}

function addInputSeasonings() {
    const input = document.createElement("input");

    input.type ="text";
    input.name = "seasonings";
    input.placeholder = "ex. Salt";
    input.className = "border-2 border-blue-500 mb-5 mr-5 rounded-sm px-4 py-2 hover:bg-gray-200 transition";

    const deleteButton = document.createElement("button");
    deleteButton.type = "button";
    deleteButton.textContent ="-";
    deleteButton.onclick = "deleteInput()";
    deleteButton.className = "border-2 border-red-400  bg-red-400 text-white rounded-sm mr-5 text-xl w-6 h-6 px-2 flex items-center justify-center leading-non hover:text-black hover:border-black transition duration-300";

    function deleteInput(){}

    const div = document.createElement("div");
    div.className = "flex justify-center items-center";

    div.appendChild(input);
    div.appendChild(deleteButton);

    const button = document.getElementById("addSeasoning");
    document.getElementById("seasonings").insertBefore(div, button);
}

function addInputCookingTools() {
    const input = document.createElement("input");

    input.type = "text";
    input.name = "cooking_tools";
    input.placeholder = "ex. Spatula";
    input.className = "border-2 border-blue-500 mb-5 mr-5 rounded-sm px-4 py-2 hover:bg-gray-200 transition";

    const deleteButton = document.createElement("button");
    deleteButton.type = "button";
    deleteButton.textContent ="-";
    deleteButton.onclick = "deleteInput()";
    deleteButton.className = "border-2 border-red-400  bg-red-400 text-white rounded-sm mr-5 text-xl w-6 h-6 px-2 flex items-center justify-center leading-non hover:text-black hover:border-black transition duration-300";

    function deleteInput(){}

    const div = document.createElement("div");
    div.className = "flex justify-center items-center";

    div.appendChild(input);
    div.appendChild(deleteButton);

    const addButton = document.getElementById("addCookingTool");
    document.getElementById("cookingTools").insertBefore(div, addButton);

}


//<button type="button" class="border-2 border-red-400  bg-red-400 text-white rounded-sm mr-5 text-xl w-6 h-6 px-2 flex items-center justify-center leading-non"> - </button>