/* * Created and Developed by: Cleiton Augusto Correa Bezerra
 */

pub mod vegas; // Declara o sub-módulo vegas.rs

// Re-exporta para que o usuário possa usar strategy::VegasStrategy
// em vez de strategy::vegas::VegasStrategy
pub use vegas::VegasStrategy;