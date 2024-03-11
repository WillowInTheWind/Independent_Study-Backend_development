using System.ComponentModel.DataAnnotations;

namespace TodoApi.Models;

public class TodoItem
{
    public long Id { get; set; }
    
    [StringLength(64, ErrorMessage = "The {0} value cannot exceed {1} characters. ")]  
    public string? Name { get; set; }
    
    public bool IsComplete { get; set; }
}