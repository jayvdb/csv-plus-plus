# frozen_string_literal: true

require_relative '../../lib/language/entities'

::FactoryBot.define do
  factory :boolean_true, class: ::CSVPlusPlus::Language::Entities::Boolean do
    initialize_with { new true }
  end

  factory :boolean_false, class: ::CSVPlusPlus::Language::Entities::Boolean do
    initialize_with { new false }
  end
end
