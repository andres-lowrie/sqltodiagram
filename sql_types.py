class Table:
    def __init__(self, name):
        """ Used as temporary representation before linking nodes with edges with graphviz.

        Args:
            name (str): Unique identifier for Graphviz.
        """
        self.name = name
        self.columns = []

        # The Dot|Graphviz representation
        self.repre = f"<f0> {self.name}"

    # def __str__(self):
    #     return f"-- {self.name} -- \n {[str(x) for x in self.columns]}"


class Column:
    def __init__(self, name, is_star=False, came_from=None, table=None):
        """ Used as temporary representation before creating the actual nodes with graphviz.

        Args:
            name (str): Unique identifier for Graphviz.
            is_star (bool): Used to identify when a 'Select *' is passed since that should result in a unique node ie:
                            the columns are no longer important in the output graph for the _that_ table
            came_from (Column): The ancestry of this Column. Used to create `edges` with Graphviz
            table (Table): Reference to the table this column belongs to. If None (default) then the "Output Table" is
                           assumed.
        """
        self.is_star = is_star
        self.name = "All" if is_star else name
        self.came_from = came_from
        self.table = table

        # The Dot|Graphviz representation
        self.repre = f"<f0> {self.name}"

    # def __str__(self):
    #     return f"-- {self.name}"
